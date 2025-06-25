//! A simple 2D renderer using `raqote` for the custom HTML-like layout engine.
//! This module handles drawing layout boxes (rectangles with style) and text.

use raqote::*;
use font_kit::source::SystemSource;
use font_kit::properties::Properties;
use font_kit::handle::Handle;
use ab_glyph::{FontArc, PxScale, GlyphId, point, Glyph};
use std::collections::HashMap;

/// A simplified color struct.
#[derive(Clone, Copy, Debug)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Color {
    pub fn to_solid(&self) -> SolidSource {
        SolidSource {
            r: self.0,
            g: self.1,
            b: self.2,
            a: self.3,
        }
    }
}

/// A rectangle box with style.
#[derive(Debug, Clone)]
pub struct LayoutBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub background: Option<Color>,
    pub border: Option<(Color, f32)>,
    pub text: Option<TextNode>,
    pub children: Vec<LayoutBox>,
}

/// Text to render inside a box
#[derive(Debug, Clone)]
pub struct TextNode {
    pub content: String,
    pub font_size: f32,
    pub color: Color,
    pub font_family: String,
}

/// The main renderer struct, which contains its DrawTarget and font cache.
pub struct Renderer {
    pub width: i32,
    pub height: i32,
    target: DrawTarget,
    font_cache: HashMap<String, FontArc>,
}

impl Renderer {
    pub fn new(width: i32, height: i32) -> Self {
        let target = DrawTarget::new(width, height);
        Self {
            width,
            height,
            target,
            font_cache: HashMap::new(),
        }
    }

    /// Clear the canvas with a solid color
    pub fn clear(&mut self, color: Color) {
        self.target.clear(color.to_solid());
    }

    /// Load a font from system or cache
    fn load_font(&mut self, family: &str) -> Option<FontArc> {
        if let Some(font) = self.font_cache.get(family) {
            return Some(font.clone());
        }

        let source = SystemSource::new();
        if let Ok(fonts) = source.select_family_by_name(family) {
            if let Ok(handle) = fonts.fonts().first().cloned().unwrap().load() {
                if let Handle::Memory { bytes, .. } = handle {
                    if let Ok(font) = FontArc::try_from_vec(bytes.to_vec()) {
                        self.font_cache.insert(family.to_string(), font.clone());
                        return Some(font);
                    }
                }
            }
        }

        None
    }

    /// Render a single layout box recursively
    pub fn render_box(&mut self, layout: &LayoutBox) {
        // Draw background
        if let Some(bg) = layout.background {
            let rect = DrawTarget::new(self.width, self.height);
            self.target.fill_rect(
                layout.x,
                layout.y,
                layout.width,
                layout.height,
                &Source::Solid(bg.to_solid()),
                &DrawOptions::new(),
            );
        }

        // Draw border
        if let Some((border_color, thickness)) = layout.border {
            let stroke_style = StrokeStyle::default();
            let source = Source::Solid(border_color.to_solid());

            let path = {
                let mut pb = PathBuilder::new();
                pb.rect(layout.x, layout.y, layout.width, layout.height);
                pb.finish()
            };

            self.target.stroke(
                &path,
                &source,
                &StrokeStyle {
                    width: thickness,
                    ..stroke_style
                },
                &DrawOptions::new(),
            );
        }

        // Render text
        if let Some(ref text) = layout.text {
            self.draw_text(
                &text.content,
                layout.x + 4.0,
                layout.y + text.font_size + 4.0,
                &text.font_family,
                text.font_size,
                text.color,
            );
        }

        // Recursively render children
        for child in &layout.children {
            self.render_box(child);
        }
    }

    /// Draw text using ab_glyph and raqote
    fn draw_text(
        &mut self,
        text: &str,
        x: f32,
        y: f32,
        font_family: &str,
        font_size: f32,
        color: Color,
    ) {
        let Some(font) = self.load_font(font_family) else {
            eprintln!("Font '{}' not found", font_family);
            return;
        };

        let scale = PxScale::from(font_size);
        let mut dx = x;

        for ch in text.chars() {
            if let Some(glyph_id) = font.glyph_id(ch) {
                let glyph = font.outline_glyph(glyph_id.with_scale(scale)).unwrap();
                if let Some(outline) = glyph.clone().into_outline() {
                    let mut pb = PathBuilder::new();
                    for segment in outline.segments() {
                        match segment {
                            ab_glyph::Segment::Line(a, b) => {
                                pb.move_to(a.x + dx, a.y + y);
                                pb.line_to(b.x + dx, b.y + y);
                            }
                            ab_glyph::Segment::Curve(a, b, c) => {
                                pb.move_to(a.x + dx, a.y + y);
                                pb.cubic_to(
                                    b.x + dx,
                                    b.y + y,
                                    c.x + dx,
                                    c.y + y,
                                    a.x + dx,
                                    a.y + y,
                                );
                            }
                        }
                    }

                    let path = pb.finish();
                    self.target.fill(
                        &path,
                        &Source::Solid(color.to_solid()),
                        &DrawOptions::new(),
                    );
                }

                dx += font.h_advance(glyph_id).unwrap_or(10.0) * font_size / 1000.0;
            }
        }
    }

    /// Export the current frame to a PNG image (debug/dev)
    pub fn save_png(&self, path: &str) {
        use std::fs::File;
        use std::io::BufWriter;
        self.target
            .write_png(BufWriter::new(File::create(path).unwrap()))
            .unwrap();
    }

    /// Get raw pixel buffer (e.g. for passing to GPU texture)
    pub fn get_data(&self) -> &[u32] {
        self.target.get_data()
    }
}

//! style.rs — Responsible for applying visual styles to the DOM.
//! This includes default tag styles, inline styles, and eventually selector-based styles.

use crate::browser::dom::{Node, NodeType, ElementData};
use crate::browser::engine::{Style, Display, Color, edges};
use std::collections::HashMap;
use std::rc::Rc;

/// Struct representing a styled DOM node with computed visual style
#[derive(Debug, Clone)]
pub struct StyledNode {
    pub node_type: NodeType,
    pub style: Style,
    pub children: Vec<StyledNode>,
}

/// Main entry point: Compute a styled tree from a DOM node
pub fn compute_styles(node: &Rc<Node>) -> StyledNode {
    let style = match &node.node_type {
        NodeType::Element(el) => compute_style_for_element(el),
        NodeType::Text(_) => default_text_style(),
        NodeType::Comment(_) => none_style(),
    };

    let children = node
        .get_children()
        .iter()
        .map(|child| compute_styles(child))
        .collect();

    StyledNode {
        node_type: node.node_type.clone(),
        style,
        children,
    }
}

/// Default text style
fn default_text_style() -> Style {
    Style {
        display: Display::Inline,
        font_size: 16.0,
        font_family: "Arial".into(),
        color: Color(0, 0, 0, 255),
        margin: edges(0.0),
        padding: edges(0.0),
        border_color: None,
        border_width: 0.0,
        background: None,
    }
}

/// Style for comments and unrendered content
fn none_style() -> Style {
    Style {
        display: Display::None,
        ..default_text_style()
    }
}

/// Assign default styles based on tag, and parse inline `style=""` attributes.
fn compute_style_for_element(el: &ElementData) -> Style {
    let mut style = match el.tag_name.as_str() {
        "body" => Style {
            display: Display::Block,
            font_size: 16.0,
            font_family: "Arial".into(),
            color: Color(30, 30, 30, 255),
            margin: edges(0.0),
            padding: edges(8.0),
            background: Some(Color(255, 255, 255, 255)),
            border_color: None,
            border_width: 0.0,
        },
        "h1" => Style {
            display: Display::Block,
            font_size: 32.0,
            font_family: "Georgia".into(),
            color: Color(50, 50, 50, 255),
            margin: edges(12.0),
            padding: edges(6.0),
            background: None,
            border_color: None,
            border_width: 0.0,
        },
        "p" => Style {
            display: Display::Block,
            font_size: 16.0,
            font_family: "Serif".into(),
            color: Color(20, 20, 20, 255),
            margin: edges(8.0),
            padding: edges(4.0),
            background: None,
            border_color: None,
            border_width: 0.0,
        },
        "div" => Style {
            display: Display::Block,
            font_size: 14.0,
            font_family: "Sans-serif".into(),
            color: Color(0, 0, 0, 255),
            margin: edges(6.0),
            padding: edges(6.0),
            background: None,
            border_color: None,
            border_width: 0.0,
        },
        _ => default_text_style(),
    };

    // Apply inline styles (e.g., <p style="color:red; background:#eee">)
    if let Some(inline_style) = el.attrs.get("style") {
        apply_inline_styles(&mut style, inline_style);
    }

    style
}

/// Parses and applies inline CSS from `style` attributes
fn apply_inline_styles(style: &mut Style, inline: &str) {
    for rule in inline.split(';') {
        if let Some((key, value)) = rule.split_once(':') {
            let key = key.trim().to_lowercase();
            let value = value.trim();

            match key.as_str() {
                "color" => {
                    if let Some(c) = parse_color(value) {
                        style.color = c;
                    }
                }
                "background" | "background-color" => {
                    if let Some(c) = parse_color(value) {
                        style.background = Some(c);
                    }
                }
                "font-size" => {
                    if let Ok(px) = value.trim_end_matches("px").parse::<f32>() {
                        style.font_size = px;
                    }
                }
                "font-family" => {
                    style.font_family = value.to_string();
                }
                "border" => {
                    if let Some((width, color)) = parse_border(value) {
                        style.border_width = width;
                        style.border_color = Some(color);
                    }
                }
                _ => {}
            }
        }
    }
}

/// Parses a basic border string: "1px solid red"
fn parse_border(value: &str) -> Option<(f32, Color)> {
    let parts: Vec<&str> = value.split_whitespace().collect();
    if parts.len() == 3 {
        if let Ok(width) = parts[0].trim_end_matches("px").parse::<f32>() {
            let color = parse_color(parts[2])?;
            return Some((width, color));
        }
    }
    None
}

/// Parses color names or hex
fn parse_color(value: &str) -> Option<Color> {
    let value = value.trim().to_lowercase();

    match value.as_str() {
        "black" => Some(Color(0, 0, 0, 255)),
        "white" => Some(Color(255, 255, 255, 255)),
        "red" => Some(Color(255, 0, 0, 255)),
        "green" => Some(Color(0, 255, 0, 255)),
        "blue" => Some(Color(0, 0, 255, 255)),
        _ if value.starts_with('#') => hex_to_color(&value[1..]),
        _ => None,
    }
}

/// Convert hex color (e.g. "ff0000") to RGB
fn hex_to_color(hex: &str) -> Option<Color> {
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Color(r, g, b, 255))
    } else {
        None
    }
}

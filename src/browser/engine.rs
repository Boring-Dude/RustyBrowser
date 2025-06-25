//! engine.rs — Layout engine: Transforms styled DOM into positioned LayoutBoxes.

use crate::browser::renderer::{LayoutBox, TextNode, Color};

/// Edge values (top, right, bottom, left)
#[derive(Debug, Clone, Copy, Default)]
pub struct EdgeSizes {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

/// Rectangle box
#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Box dimensions (padding, borders, etc.)
#[derive(Debug, Clone, Default)]
pub struct Dimensions {
    pub content: Rect,
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}

/// Display types supported
#[derive(Debug, Clone, PartialEq)]
pub enum Display {
    Block,
    Inline,
    None,
}

/// Style associated with a node
#[derive(Debug, Clone)]
pub struct Style {
    pub display: Display,
    pub background: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: f32,
    pub margin: EdgeSizes,
    pub padding: EdgeSizes,
    pub font_size: f32,
    pub font_family: String,
    pub color: Color,
}

/// DOM element types
#[derive(Debug, Clone)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug, Clone)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: Vec<(String, String)>,
}

/// Renderable DOM node (styled and ready for layout)
#[derive(Debug, Clone)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
    pub style: Style,
}

pub fn build_layout_tree(node: &Node, container_width: f32) -> LayoutBox {
    let mut root_dimensions = Dimensions::default();
    root_dimensions.content.width = container_width;

    build_layout_box(node, root_dimensions, 0.0, 0.0)
}

fn build_layout_box(node: &Node, container: Dimensions, offset_x: f32, mut offset_y: f32) -> LayoutBox {
    let style = normalize_style(node.style.clone());

    if style.display == Display::None {
        return LayoutBox::empty();
    }

    let x = offset_x + style.margin.left + style.border_width + style.padding.left;
    let mut y = offset_y + style.margin.top + style.border_width + style.padding.top;

    let width = container.width
        - (style.margin.left + style.margin.right + style.padding.left + style.padding.right + 2.0 * style.border_width)
        .max(0.0);

    let mut height = 0.0;
    let mut children = vec![];

    for child in &node.children {
        
    }
}

fn normalize(mut style: Style) -> Style {
    style.border_width = style.border_width.max(0.0);
    style.font_size = style.font_size.max(1.0);
    style
}
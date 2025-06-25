//! The layout and rendering engine for the RustyBrowser
//! This file turns DOM-like nodes into positioned `LayoutBox` structures.

use crate::browser::renderer::{LayoutBox, TextNode, Color};

/// Box model dimensions
#[derive(Debug, Clone, Default)]
pub struct Dimensions {
    pub content: Rect,
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}

/// Rectangle for layout
#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Edge values (top, right, bottom, left)
#[derive(Debug, Clone, Copy, Default)]
pub struct EdgeSizes {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

/// Simplified style object (normally comes from CSS parser)
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

#[derive(Debug, Clone, PartialEq)]
pub enum Display {
    Block,
    Inline,
    None,
}

/// Simplified DOM node structure (just tag and content)
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

/// The actual DOM Node
#[derive(Debug, Clone)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
    pub style: Style,
}

/// Build a layout tree from styled DOM nodes
pub fn build_layout_tree(node: &Node, container_width: f32) -> LayoutBox {
    let mut dimensions = Dimensions::default();
    dimensions.content.width = container_width;

    let mut layout = build_layout_box(node, dimensions, 0.0, 0.0);
    layout
}

/// Recursively build layout box tree with positions and sizes
fn build_layout_box(node: &Node, mut container: Dimensions, offset_x: f32, mut offset_y: f32) -> LayoutBox {
    // Skip display: none
    if node.style.display == Display::None {
        return LayoutBox {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            background: None,
            border: None,
            text: None,
            children: vec![],
        };
    }

    let mut box_x = offset_x
        + node.style.margin.left
        + node.style.border_width
        + node.style.padding.left;

    let mut box_y = offset_y
        + node.style.margin.top
        + node.style.border_width
        + node.style.padding.top;

    let mut width = container.content.width
        - (node.style.margin.left
            + node.style.margin.right
            + node.style.padding.left
            + node.style.padding.right
            + node.style.border_width * 2.0);

    let mut height = 0.0;
    let mut children_boxes = vec![];

    // Recursively layout children
    for child in &node.children {
        let child_box = build_layout_box(
            child,
            container.clone(),
            box_x,
            box_y + height,
        );
        height += child_box.height
            + node.style.margin.top
            + node.style.margin.bottom
            + node.style.padding.top
            + node.style.padding.bottom;

        children_boxes.push(child_box);
    }

    // If text node, calculate height based on font size
    let mut text_node = None;
    if let NodeType::Text(ref text_content) = node.node_type {
        height += node.style.font_size + node.style.padding.top + node.style.padding.bottom;

        text_node = Some(TextNode {
            content: text_content.clone(),
            font_size: node.style.font_size,
            color: node.style.color,
            font_family: node.style.font_family.clone(),
        });
    }

    // Total height if no children
    if children_boxes.is_empty() && text_node.is_none() {
        height += 20.0; // default block height
    }

    LayoutBox {
        x: box_x,
        y: box_y,
        width,
        height,
        background: node.style.background,
        border: node
            .style
            .border_color
            .map(|c| (c, node.style.border_width)),
        text: text_node,
        children: children_boxes,
    }
}

/// Utility: create edge sizes from uniform value
pub fn edges(value: f32) -> EdgeSizes {
    EdgeSizes {
        top: value,
        right: value,
        bottom: value,
        left: value,
    }
}

/// Utility: parse sample style for mock DOM (used in prototyping)
pub fn default_style(display: Display) -> Style {
    Style {
        display,
        background: Some(Color(240, 240, 240, 255)),
        border_color: Some(Color(0, 0, 0, 255)),
        border_width: 1.0,
        margin: edges(8.0),
        padding: edges(8.0),
        font_size: 16.0,
        font_family: "Arial".to_string(),
        color: Color(0, 0, 0, 255),
    }
}

/// Sample node builder for testing layout (until parser is ready)
pub fn sample_dom_tree() -> Node {
    Node {
        style: default_style(Display::Block),
        node_type: NodeType::Element(ElementData {
            tag_name: "div".to_string(),
            attributes: vec![],
        }),
        children: vec![
            Node {
                style: default_style(Display::Block),
                node_type: NodeType::Text("Hello, Zen!".to_string()),
                children: vec![],
            },
            Node {
                style: Style {
                    display: Display::Block,
                    background: Some(Color(220, 220, 250, 255)),
                    border_color: Some(Color(0, 0, 128, 255)),
                    border_width: 1.5,
                    margin: edges(4.0),
                    padding: edges(6.0),
                    font_size: 14.0,
                    font_family: "Courier New".to_string(),
                    color: Color(20, 20, 20, 255),
                },
                node_type: NodeType::Text("Welcome to the Rust Browser Engine!".to_string()),
                children: vec![],
            },
        ],
    }
}

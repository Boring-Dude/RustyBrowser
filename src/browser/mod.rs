//! The main browser module
//! It defines the core structure of a lightweight HTML/CSS renderer,
//! including the DOM tree, parser, style system, layout engine, and renderer.

pub mod dom;
pub mod parser;
pub mod style;
pub mod engine;
pub mod renderer;

// Export key types and functions for external use
// This acts like the browser's public API

// === DOM Tree ===
pub use dom::{
    Node, NodeType, ElementData, AttrMap,
    element, text, comment, print_tree,
};

// === HTML Parser ===
pub use parser::parse_html;

// === Style System ===
pub use style::{StyledNode, compute_styles};

// === Layout & Engine ===
pub use engine::{
    Node as LayoutNode, Style, Display, Color,
    Dimensions, Rect, EdgeSizes,
    build_layout_tree, default_style, edges,
};

// === Renderer ===
pub use renderer::{Renderer, LayoutBox, TextNode};

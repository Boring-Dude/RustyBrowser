//! A simplified DOM (Document Object Model) structure
//! Used for representing the HTML tree before layout/rendering

use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::{Rc, Weak};

/// The type of a node in the DOM
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

/// Represents one node in the DOM tree
#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub children: RefCell<Vec<Rc<Node>>>,
    pub parent: RefCell<Option<Weak<Node>>>,
}

impl Node {
    /// Create a new DOM node
    pub fn new(node_type: NodeType) -> Rc<Node> {
        Rc::new(Node {
            node_type,
            children: RefCell::new(vec![]),
            parent: RefCell::new(None),
        })
    }

    /// Append a child node
    pub fn append_child(parent: &Rc<Node>, child: Rc<Node>) {
        child.parent.borrow_mut().replace(Rc::downgrade(parent));
        parent.children.borrow_mut().push(child);
    }

    /// Get a borrowed reference to the children
    pub fn get_children(&self) -> Ref<Vec<Rc<Node>>> {
        self.children.borrow()
    }

    /// Get a mutable reference to the children
    pub fn get_children_mut(&self) -> RefMut<Vec<Rc<Node>>> {
        self.children.borrow_mut()
    }

    /// Get tag name if this is an element node
    pub fn tag_name(&self) -> Option<&str> {
        match &self.node_type {
            NodeType::Element(el) => Some(&el.tag_name),
            _ => None,
        }
    }

    /// Get text content if it's a text node
    pub fn text_content(&self) -> Option<&str> {
        match &self.node_type {
            NodeType::Text(text) => Some(text),
            _ => None,
        }
    }

    /// Get attribute value if present
    pub fn get_attr(&self, name: &str) -> Option<String> {
        match &self.node_type {
            NodeType::Element(el) => el.attrs.get(name).cloned(),
            _ => None,
        }
    }

    /// Set or replace an attribute
    pub fn set_attr(&mut self, name: &str, value: &str) {
        if let NodeType::Element(el) = &mut self.node_type {
            el.attrs.insert(name.to_string(), value.to_string());
        }
    }
}

/// Information specific to element nodes
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementData {
    pub tag_name: String,
    pub attrs: AttrMap,
}

pub type AttrMap = HashMap<String, String>;

/// Helper to create an element node
pub fn element(tag_name: &str, attrs: AttrMap, children: Vec<Rc<Node>>) -> Rc<Node> {
    let node = Node::new(NodeType::Element(ElementData {
        tag_name: tag_name.to_lowercase(),
        attrs,
    }));

    for child in children {
        Node::append_child(&node, child);
    }

    node
}

/// Helper to create a text node
pub fn text(content: &str) -> Rc<Node> {
    Node::new(NodeType::Text(content.to_string()))
}

/// Helper to create a comment node
pub fn comment(content: &str) -> Rc<Node> {
    Node::new(NodeType::Comment(content.to_string()))
}

/// Recursively pretty-print the DOM tree (debugging)
pub fn print_tree(node: &Rc<Node>, indent: usize) {
    for _ in 0..indent {
        print!("  ");
    }

    match &node.node_type {
        NodeType::Text(text) => println!("Text: \"{}\"", text),
        NodeType::Comment(comment) => println!("<!-- {} -->", comment),
        NodeType::Element(el) => {
            println!("<{}>", el.tag_name);
            for child in node.get_children().iter() {
                print_tree(child, indent + 1);
            }
            for _ in 0..indent {
                print!("  ");
            }
            println!("</{}>", el.tag_name);
        }
    }
}

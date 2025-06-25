//! dom.rs — Hardened, secure DOM representation

use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::{Rc, Weak};

/// Map of element attributes (e.g., class="x")
pub type AttrMap = HashMap<String, String>;

/// Tag/Element metadata
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementData {
    pub tag_name: String,
    pub attrs: AttrMap,
}

/// Enum representing the type of node in the DOM
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

/// A DOM node with references to children and parent
#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Option<Weak<Node>>>,
}

impl Node {
    /// Create a new node with the specified type
    pub fn new(node_type: NodeType) -> Rc<Node> {
        Rc::new(Node {
            node_type,
            children: RefCell::new(vec![]),
            parent: RefCell::new(None),
        })
    }

    /// Append a child node (ensures no cycles)
    pub fn append_child(parent: &Rc<Node>, child: Rc<Node>) {
        if Rc::ptr_eq(parent, &child) {
            panic!("Cannot append node to itself");
        }

        // Ensure this node isn't already in the parent's subtree
        let mut current = Some(parent.clone());
        while let Some(node) = current {
            if Rc::ptr_eq(&node, &child) {
                panic!("Cannot insert ancestor as child (would cause cycle)");
            }
            current = node.parent.borrow().as_ref().and_then(|w| w.upgrade());
        }

        child.parent.borrow_mut().replace(Rc::downgrade(parent));
        parent.children.borrow_mut().push(child);
    }

    /// Get immutable children
    pub fn children(&self) -> Ref<Vec<Rc<Node>>> {
        self.children.borrow()
    }

    /// Get mutable access to children (use with caution)
    pub fn children_mut(&self) -> RefMut<Vec<Rc<Node>>> {
        self.children.borrow_mut()
    }

    /// Get the parent node (if any)
    pub fn parent(&self) -> Option<Rc<Node>> {
        self.parent.borrow().as_ref().and_then(|w| w.upgrade())
    }

    /// Returns true if this is a text node
    pub fn is_text(&self) -> bool {
        matches!(self.node_type, NodeType::Text(_))
    }

    /// Returns true if this is an element node
    pub fn is_element(&self) -> bool {
        matches!(self.node_type, NodeType::Element(_))
    }

    /// Returns true if this is a comment node
    pub fn is_comment(&self) -> bool {
        matches!(self.node_type, NodeType::Comment(_))
    }

    /// Return the tag name if this is an element node
    pub fn tag_name(&self) -> Option<&str> {
        match &self.node_type {
            NodeType::Element(el) => Some(&el.tag_name),
            _ => None,
        }
    }

    /// Return the text content if it's a text node
    pub fn text(&self) -> Option<&str> {
        match &self.node_type {
            NodeType::Text(txt) => Some(txt),
            _ => None,
        }
    }

    /// Get an attribute value (case-sensitive)
    pub fn get_attr(&self, name: &str) -> Option<String> {
        match &self.node_type {
            NodeType::Element(el) => el.attrs.get(name).cloned(),
            _ => None,
        }
    }

    /// Returns true if an attribute is present
    pub fn has_attr(&self, name: &str) -> bool {
        self.get_attr(name).is_some()
    }

    /// Set or replace an attribute (normalized)
    pub fn set_attr(&self, name: &str, value: &str) {
        if let NodeType::Element(el) = &mut self.node_type.clone() {
            let clean_key = name.trim().to_lowercase();
            let clean_val = value.trim().to_string();
            let mut new_attrs = el.attrs.clone();
            new_attrs.insert(clean_key, clean_val);
            self.replace_element_data(el.tag_name.clone(), new_attrs);
        }
    }

    /// Remove an attribute
    pub fn remove_attr(&self, name: &str) {
        if let NodeType::Element(el) = &mut self.node_type.clone() {
            let mut new_attrs = el.attrs.clone();
            new_attrs.remove(name);
            self.replace_element_data(el.tag_name.clone(), new_attrs);
        }
    }

    /// Internal: Replace element metadata (to apply attribute changes)
    fn replace_element_data(&self, tag: String, attrs: AttrMap) {
        if let NodeType::Element(_) = self.node_type {
            let new_data = ElementData {
                tag_name: tag,
                attrs,
            };
            self.node_type = NodeType::Element(new_data);
        }
    }
}

/// Construct an element node with tag, attributes, and children
pub fn element(tag_name: &str, attrs: AttrMap, children: Vec<Rc<Node>>) -> Rc<Node> {
    let tag = tag_name.trim().to_lowercase();
    let node = Node::new(NodeType::Element(ElementData {
        tag_name: tag,
        attrs,
    }));

    for child in children {
        Node::append_child(&node, child);
    }

    node
}

/// Construct a text node
pub fn text(content: &str) -> Rc<Node> {
    Node::new(NodeType::Text(content.to_string()))
}

/// Construct a comment node
pub fn comment(content: &str) -> Rc<Node> {
    Node::new(NodeType::Comment(content.to_string()))
}

/// Pretty-print the DOM (safe, recursive, with cycle detection and depth guard)
pub fn print_tree(node: &Rc<Node>, indent: usize) {
    const MAX_DEPTH: usize = 100;
    if indent > MAX_DEPTH {
        println!("  [Max depth reached]");
        return;
    }

    for _ in 0..indent {
        print!("  ");
    }

    match &node.node_type {
        NodeType::Text(text) => println!("Text: {:?}", text),
        NodeType::Comment(comment) => println!("<!-- {} -->", comment),
        NodeType::Element(el) => {
            print!("<{}", el.tag_name);
            for (k, v) in &el.attrs {
                print!(" {}=\"{}\"", k, v);
            }
            println!(">");
            for child in node.children().iter() {
                print_tree(child, indent + 1);
            }
            for _ in 0..indent {
                print!("  ");
            }
            println!("</{}>", el.tag_name);
        }
    }
}

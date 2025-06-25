//! A very basic HTML parser that turns raw HTML into a DOM tree.
//! It builds a simplified `Node` tree based on HTML structure.

use crate::browser::dom::{Node, NodeType, ElementData, element, text};
use std::collections::HashMap;

#[derive(Debug)]
pub struct HTMLParser {
    pos: usize,
    input: String,
}

impl HTMLParser {
    pub fn new(input: &str) -> Self {
        Self {
            pos: 0,
            input: input.to_string(),
        }
    }

    pub fn parse(&mut self) -> Node {
        let mut nodes = self.parse_nodes();
        if nodes.len() == 1 {
            Rc::try_unwrap(nodes.remove(0)).unwrap_or_else(|rc| (*rc).clone())
        } else {
            element("html", HashMap::new(), nodes)
        }
    }

    fn parse_nodes(&mut self) -> Vec<Rc<Node>> {
        let mut nodes = Vec::new();
        self.consume_whitespace();
        while !self.eof() && !self.starts_with("</") {
            nodes.push(self.parse_node());
            self.consume_whitespace();
        }
        nodes
    }

    fn parse_node(&mut self) -> Rc<Node> {
        if self.starts_with("<") {
            self.parse_element()
        } else {
            self.parse_text()
        }
    }

    fn parse_text(&mut self) -> Rc<Node> {
        let text = self.consume_while(|c| c != '<');
        text(&text)
    }

    fn parse_element(&mut self) -> Rc<Node> {
        assert!(self.consume_char() == '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        let children = self.parse_nodes();

        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        let close_tag = self.parse_tag_name();
        assert!(close_tag == tag_name);
        assert!(self.consume_char() == '>');

        element(&tag_name, attrs, children)
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| c.is_alphanumeric())
    }

    fn parse_attributes(&mut self) -> HashMap<String, String> {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.current_char() == '>' {
                break;
            }

            let name = self.parse_tag_name();
            self.consume_whitespace();
            assert!(self.consume_char() == '=');
            self.consume_whitespace();
            let value = self.parse_attr_value();
            attributes.insert(name, value);
        }
        attributes
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert!(self.consume_char() == open_quote);
        value
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, current) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        current
    }

    fn current_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap_or('\0')
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.current_char()) {
            result.push(self.consume_char());
        }
        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}

use std::rc::Rc;

/// Parse HTML into a DOM `Node` tree
pub fn parse_html(input: &str) -> Rc<Node> {
    let mut parser = HTMLParser::new(input);
    Rc::new(parser.parse())
}

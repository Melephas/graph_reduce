use std::rc::{
    Rc,
    Weak,
};

// TODO: Document this
pub struct Node {
    value: String,
    children: Vec<Weak<Node>>,
}

impl Node {
    // TODO: Document this
    pub fn new() -> Node {
        Node {
            value: String::new(),
            children: Vec::new(),
        }
    }

    // TODO: Document this
    pub fn with_value(value: String) -> Node {
        Node {
            value,
            children: Vec::new(),
        }
    }

    // TODO: Document this
    pub fn add_link_to_node(&mut self, node: Weak<Node>) {
        self.children.push(node);
    }
}

use super::node::Node;

#[derive(Debug)]
pub struct KBucket {
    pub nodes: Vec<Node>,
    pub size: usize,
}

impl KBucket {
    pub fn new(size: usize) -> Self {
        Self {
            nodes: vec![],
            size,
        }
    }
}

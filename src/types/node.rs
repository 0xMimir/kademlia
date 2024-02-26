use super::key::Key;

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Debug)]
/// In white paper this should also contain address
pub struct Node {
    pub port: u16,
    pub id: Key,
}

impl Node {
    pub fn new(port: u16, id: Key) -> Self {
        Node {
            port,
            id,
        }
    }
    
    pub fn get_addr(&self) -> String {
        format!("192.168.1.102:{}", self.port)
    }
}

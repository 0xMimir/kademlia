use super::{distance::NodeDistance, key::Key};

#[derive(Serialize, Deserialize)]
/// this should have same enum variants as [`Response`] with different values
pub enum Request {
    Ping,          // PING
    FindNode(Key), // FIND_NODE
}

#[derive(Serialize, Deserialize)]
/// to prevent possible errors this should have GET and PUT variants too
pub enum Response {
    Pong,
    FindNode(Vec<NodeDistance>),
}

#[derive(Serialize, Deserialize)]
pub enum Message {
    Request(Request),
    Response(Response),
}

#[derive(Serialize, Deserialize)]
pub struct RpcMessage {
    pub token: usize,
    pub source: u16, // Source should be node
    pub message: Message,
}

pub struct RpcRequest {
    pub token: usize,
    pub source: u16, // Source should be node
    pub payload: Request,
}

impl RpcMessage {
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("Error serializing")
    }

    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes).expect("Error serializing")
    }
}

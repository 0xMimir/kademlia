use crate::{
    helpers::ExpectLock,
    types::{
        messages::{Message, Request, Response, RpcMessage, RpcRequest},
        node::Node,
    },
};

use rand::{thread_rng, Rng};
use std::{
    collections::HashMap,
    net::UdpSocket,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

#[derive(Clone)]
pub struct NetworkInterface {
    socket: Arc<UdpSocket>,
    in_progress: Arc<Mutex<HashMap<usize, mpsc::Sender<Option<Response>>>>>,
    node: Node,
}

impl NetworkInterface {
    pub fn new(node: Node) -> Self {
        let socket = UdpSocket::bind(node.get_addr()).expect("Error binding");

        Self {
            socket: Arc::new(socket),
            in_progress: Arc::new(Mutex::new(HashMap::new())),
            node,
        }
    }

    /// this should be moved (not handled by kademlia)
    pub fn spawn(self, sender: mpsc::Sender<RpcRequest>) {
        thread::spawn(move || {
            let mut buf = [0u8; 4096]; // somewhere

            loop {
                let (len, _) = self
                    .socket
                    .recv_from(&mut buf)
                    .expect("Error reading from socket");

                let RpcMessage {
                    token,
                    source,
                    message,
                    ..
                } = RpcMessage::from_bytes(&buf[..len]);

                match message {
                    Message::Request(request) => {
                        let wrapped_req = RpcRequest {
                            token,
                            source,
                            payload: request,
                        };

                        if sender.send(wrapped_req).is_err() {
                            error!("Unable to use channel");
                            break;
                        }
                    }
                    Message::Response(response) => {
                        let socket = self.clone();
                        thread::spawn(move || {
                            let mut pending = socket.in_progress.expect_lock();

                            let Some(sender) = pending.get(&token) else {
                                warn!("Received invalid token"); // this will also happen if response is longer than timeout
                                return;
                            };

                            // Handle failed here
                            if sender.send(Some(response)).is_ok() {
                                pending.remove(&token);
                            }
                        });
                    }
                }
            }
        });
    }

    pub fn send_msg(&self, msg: RpcMessage, destination: u16) {
        let encoded = msg.to_bytes();
        self.socket
            .send_to(&encoded, format!("{}:{}", env!("IP_ADDR"), destination))
            .expect("Error sending");
    }

    pub fn request(&self, request: Request, destination: Node) -> mpsc::Receiver<Option<Response>> {
        let (sender, receiver) = mpsc::channel(); // this should be oneshot channel with timeout handled properly

        let mut pending = self.in_progress.expect_lock();

        let mut rng = thread_rng();
        let token = rng.gen_range(0..10000_usize);

        pending.insert(token, sender.clone());

        self.send_msg(
            RpcMessage {
                token,
                source: self.node.port,
                message: Message::Request(request),
            },
            destination.port,
        );

        let rpc = self.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1)); // Time to wait for response
            if sender.send(None).is_ok() {
                error!("Unable to send message");
            }
            rpc.in_progress.expect_lock().remove(&token);
        });

        receiver
    }
}

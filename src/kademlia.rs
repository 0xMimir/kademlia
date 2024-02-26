use crate::{
    helpers::ExpectLock,
    table,
    socket::NetworkInterface,
    types::{
        distance::NodeDistance,
        key::Key,
        messages::{Message, Request, Response, RpcMessage, RpcRequest},
        node::Node,
    },
};

use std::{
    collections::{BinaryHeap, HashSet},
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

#[derive(Clone, Copy)]
#[allow(unused)]
struct KademliaConfig {
    key_length: usize,
    n_buckets: usize,
    k_param: usize,
    alpha: usize,
}

impl Default for KademliaConfig {
    fn default() -> Self {
        Self {
            key_length: 32,
            n_buckets: 32 * 8,
            k_param: 20,
            alpha: 3,
        }
    }
}

#[derive(Clone)]
/// Clone should be removed with Arc and Mutex
/// and replaced with some sort of queue
pub struct Kademlia {
    routes: Arc<Mutex<table::RoutingTable>>,
    rpc: Arc<NetworkInterface>,
    node: Node,
    config: KademliaConfig,
}

impl Kademlia {
    pub fn new(port: u16, peer_id: Key) -> Self {
        let node = Node::new(port, peer_id);
        let config = KademliaConfig::default();

        let routes = table::RoutingTable::new(node, config.n_buckets, config.k_param);
        
        let (rpc_sender, rpc_receiver) = mpsc::channel();

        let rpc = NetworkInterface::new(node);
        rpc.clone().spawn(rpc_sender);

        let kademlia = Self {
            routes: Arc::new(Mutex::new(routes)),
            rpc: Arc::new(rpc),
            node,
            config,
        };

        let protocol = kademlia.clone();
        // this should be moved to listen function or something like that
        std::thread::spawn(move || {
            while let Ok(request) = rpc_receiver.recv() {
                let k = protocol.clone();
                std::thread::spawn(move || k.respond(request));
            }
        });

        // dbg!(&kademlia.routes.expect_lock());
        kademlia
    }

    fn respond(&self, request: RpcRequest) {
        {
            let mut routes = self.routes.expect_lock();
            let source_node = Node::new(request.source, Key::new(request.source.to_string()));
            routes.update(source_node); // Add node that request to know nodes
        }

        let response = match request.payload {
            Request::Ping => Response::Pong,
            Request::FindNode(ref id) => {
                let routes = self.routes.expect_lock();
                let result = routes.get_closest_nodes(id, self.config.k_param);
                Response::FindNode(result)
            }
        };

        let msg = RpcMessage {
            token: request.token,
            source: self.node.port,
            message: Message::Response(response),
        };

        self.rpc.send_msg(msg, request.source);
    }

    pub fn bootstrap(&mut self, node: Node) {
        self.routes.expect_lock().update(node);
        self.lookup_nodes(&self.node.id);
    }

    pub fn node(&self) -> &Node {
        &self.node
    }

    pub fn get_all_know_nodes(&self) -> Vec<Node> {
        self.routes
            .expect_lock()
            .get_kbuckets()
            .iter()
            .flat_map(|bucket| bucket.nodes.clone())
            .collect()
    }

    pub fn ping(&self, dst: Node) -> bool {
        let response = self
            .rpc
            .request(Request::Ping, dst)
            .recv()
            .expect("Error making request");

        let mut routes = self.routes.expect_lock();

        if let Some(Response::Pong) = response {
            routes.update(dst);
            true
        } else {
            error!("No pong from peer: {}:{}", dst.id, dst.port);
            routes.remove(&dst.id);
            false
        }
    }

    pub fn find_node(&self, dst: Node, id: Key) -> Option<Vec<NodeDistance>> {
        let response = self
            .rpc
            .request(Request::FindNode(id), dst)
            .recv()
            .expect("Error making request");

        let mut routes = self.routes.expect_lock();

        if let Some(Response::FindNode(entries)) = response {
            routes.update(dst);
            Some(entries)
        } else {
            routes.remove(&dst.id);
            None
        }
    }

    pub fn lookup_nodes(&self, id: &Key) -> Vec<NodeDistance> {
        let mut nodes = vec![];

        let mut to_query = {
            let routes = self.routes.expect_lock();
            BinaryHeap::from(routes.get_closest_nodes(id, self.config.k_param))
        };
        let mut queried = to_query.iter().map(Clone::clone).collect::<HashSet<_>>();

        while !to_query.is_empty() {
            let queries = (0..self.config.alpha)
                .filter_map(|_| to_query.pop())
                .collect::<Vec<_>>();

            let threads = queries
                .iter()
                .map(|NodeDistance { ref node, .. }| {
                    let node = *node;
                    let node_id = *id;
                    let protocol = self.clone();

                    thread::spawn(move || protocol.find_node(node, node_id))
                })
                .collect::<Vec<_>>();

            threads
                .into_iter()
                .map(JoinHandle::join)
                .filter_map(|r| r.ok().flatten())
                .zip(queries)
                .for_each(|(entries, query)| {
                    nodes.push(query);

                    for entry in entries {
                        if queried.insert(entry.clone()) {
                            to_query.push(entry);
                        }
                    }
                });
        }

        nodes.sort_by(|a, b| a.distance.cmp(&b.distance));
        nodes.truncate(self.config.k_param);

        nodes
    }
}

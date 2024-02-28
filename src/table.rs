// use std::sync::mpsc;

use crate::types::{distance::NodeDistance, kbucket::KBucket, key::Key, node::Node};

#[derive(Debug)]
pub struct RoutingTable {
    node: Node,
    kbuckets: Vec<KBucket>,
    k_param: usize,
}

// struct RoutingTableInner {
//     node: Node,
//     kbuckets: Vec<KBucket>,
//     k_param: usize,
// }

// pub struct RoutingTableChannels<T> {
//     receiver: mpsc::Receiver<T>,
//     sender: mpsc::Sender<T>,
// }

impl RoutingTable {
    pub fn new(node: Node, n_buckets: usize, k_param: usize) -> Self {
        let kbuckets = (0..n_buckets).map(KBucket::new).collect::<Vec<_>>();

        Self {
            node,
            kbuckets,
            k_param,
        }
    }

    pub fn get_kbuckets(&self) -> &[KBucket] {
        &self.kbuckets
    }

    pub fn update(&mut self, node: Node) {
        let bucket_index = crate::pure::bucket_index(&self.node.id, &node.id);

        if self.kbuckets[bucket_index].nodes.len() < self.k_param {
            let node_index = self.kbuckets[bucket_index]
                .nodes
                .iter()
                .position(|x| x.id == node.id);
            match node_index {
                Some(i) => {
                    self.kbuckets[bucket_index].nodes.remove(i);
                    self.kbuckets[bucket_index].nodes.push(node);
                }
                None => {
                    self.kbuckets[bucket_index].nodes.push(node);
                }
            }
        } else {
            // add to pending nodes or known nodes
            // code that pings nodes need to be added
        }
    }

    pub fn remove(&mut self, node_id: &Key) {
        let bucket_index = super::pure::bucket_index(&self.node.id, node_id);

        if let Some(i) = self.kbuckets[bucket_index]
            .nodes
            .iter()
            .position(|x| &x.id == node_id)
        {
            self.kbuckets[bucket_index].nodes.remove(i);
        } else {
            error!("Removing node that is not in state")
        }
    }

    // count only for testing will later be replaced
    pub fn get_closest_nodes(&self, key: &Key, count: usize) -> Vec<NodeDistance> {
        if count == 0 {
            return vec![];
        }

        let mut ret = Vec::with_capacity(count);

        let mut bucket_index = crate::pure::bucket_index(&self.node.id, key);
        let mut bucket_index_copy = bucket_index;

        ret.extend(self.kbuckets[bucket_index].nodes.iter().map(|node| {
            let distance = node.id.distance(key);
            NodeDistance::new(*node, distance)
        }));

        while ret.len() < count && bucket_index < self.kbuckets.len() - 1 {
            bucket_index += 1;

            ret.extend(self.kbuckets[bucket_index].nodes.iter().map(|node| {
                let distance = node.id.distance(key);
                NodeDistance::new(*node, distance)
            }))
        }

        while ret.len() < count && bucket_index_copy > 0 {
            bucket_index_copy -= 1;

            ret.extend(self.kbuckets[bucket_index_copy].nodes.iter().map(|node| {
                let distance = node.id.distance(key);
                NodeDistance::new(*node, distance)
            }))
        }

        ret.sort_by(|a, b| a.distance.cmp(&b.distance));
        ret.truncate(count);
        ret
    }
}

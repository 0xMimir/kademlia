use crate::KEY_SIZE;

use super::{key::Key, node::Node};

#[derive(Clone, Serialize, Deserialize, Hash, Ord, PartialOrd, Eq, PartialEq, Copy, Debug)]
pub struct Distance(pub [u8; KEY_SIZE]);

impl Distance {
    pub fn new(k1: &Key, k2: &Key) -> Distance {
        let mut distance = [0; KEY_SIZE];

        for (index, d) in distance.iter_mut().enumerate().take(KEY_SIZE) {
            *d = k1.0[index] ^ k2.0[index];
        }

        Self(distance)
    }
}

#[derive(Serialize, Deserialize, Eq, Hash, Clone, Debug)]
pub struct NodeDistance {
    pub node: Node,
    pub distance: Distance,
}

impl NodeDistance {
    pub fn new(node: Node, distance: Distance) -> Self {
        Self { node, distance }
    }
}

impl PartialEq for NodeDistance {
    fn eq(&self, other: &NodeDistance) -> bool {
        let mut equal = true;
        let mut i = 0;
        while equal && i < 32 {
            if self.distance.0[i] != other.distance.0[i] {
                equal = false;
            }

            i += 1;
        }

        equal
    }
}

impl PartialOrd for NodeDistance {
    fn partial_cmp(&self, other: &NodeDistance) -> Option<std::cmp::Ordering> {
        Some(other.distance.cmp(&self.distance))
    }
}

impl Ord for NodeDistance {
    fn cmp(&self, other: &NodeDistance) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

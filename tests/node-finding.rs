#![allow(unused)]

use kademlia::{Kademlia, Key, Node};
use log::{error, info};

const NODE_COUNT: usize = 64;

#[test]
fn node_finding() {
    env_logger::init();
    let mut nodes = Vec::with_capacity(NODE_COUNT);

    for node in 0..NODE_COUNT {
        nodes.push(Kademlia::new(
            (10000 + node) as u16,
            Key::new((10000 + node).to_string()),
        ));
    }

    println!("Created {} nodes", NODE_COUNT);

    for node in nodes.iter() {
        assert_eq!(
            node.get_all_know_nodes().len(),
            1,
            "Nodes shouldn't have any node but itself"
        );
        assert_eq!(
            node.get_all_know_nodes().get(0).unwrap(),
            node.node(),
            "Nodes only node should be itself"
        );
    }

    let seed_node = Kademlia::new(
        (10000 + NODE_COUNT) as u16,
        Key::new((10000 + NODE_COUNT).to_string()),
    );

    for (_pos, node) in nodes.iter_mut().enumerate() {
        assert!(seed_node.ping(node.node().clone()));
        assert_eq!(node.get_all_know_nodes().len(), 2);
    }

    let mut new_node = Kademlia::new(
        (10000 + NODE_COUNT + 1) as u16,
        Key::new((10000 + NODE_COUNT + 1).to_string()),
    );
    let new_node_id = new_node.node().id;
    dbg!(seed_node.get_all_know_nodes().len());

    assert_eq!(
        new_node.get_all_know_nodes().len(),
        1,
        "There should be only one know node, itself"
    );
    new_node.ping(seed_node.node().clone());
    assert_eq!(
        new_node.get_all_know_nodes().len(),
        2,
        "There should be two nodes seed and itself"
    );

    for (_pos, node) in nodes.iter_mut().enumerate() {
        let f = node.find_node(*seed_node.node(), new_node_id);
        assert!(f.is_some(), "Node should be able to find any node");
        node.bootstrap(*seed_node.node());
    }

    dbg!(new_node.get_all_know_nodes().len());
    // new_node.bootstrap(*new_node.node());
    // dbg!(new_node.get_all_know_nodes().len());
    assert!(
        new_node.get_all_know_nodes().len() > 2,
        "At least 1 node should have connected to node in process"
    );

    let response = new_node.ping(Node::new(9999, Key::new(9999.to_string())));
    assert!(!response, "This node should not exist");
    // panic!("Panic to see debugs")
}

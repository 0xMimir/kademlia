#![cfg(not(tarpaulin_include))]

use std::{
    env,
    io::{stdin, stdout, Write},
};
use strum::{Display, EnumString};

use kademlia::{Kademlia, Key, Node};

#[derive(EnumString, Display)]
enum Command {
    #[strum(serialize = "add_peer")]
    AddPeer,
    #[strum(serialize = "print")]
    Print,
    #[strum(serialize = "bootstrap")]
    BootStrap,
}

fn main() {
    let port = env::args()
        .last()
        .expect("Port must be provided")
        .parse()
        .expect("Error parsing");

    let mut kademlia = Kademlia::new(port, Key::new(port.to_string()));

    loop {
        let command = get_command();

        match command {
            Command::AddPeer => {
                let port = read("Enter port: ").parse().expect("Invalid port");
                kademlia.ping(Node::new(port, Key::new(port.to_string())));
            }
            Command::Print => {
                let peers = kademlia.get_all_know_nodes();
                for peer in peers {
                    println!("{}", peer.id);
                    read("");
                }
            }
            Command::BootStrap => {
                let port = read("Enter port: ").parse().expect("Invalid port");
                kademlia.bootstrap(Node::new(port, Key::new(port.to_string())));
            }
        }
    }
}

fn get_command() -> Command {
    loop {
        let input = read(":> ");
        if let Ok(command) = input.parse() {
            return command;
        }

        println!("Invalid command: {}", input);
    }
}

fn read(message: &str) -> String {
    let mut command = String::new();
    print!("{}", message);
    stdout().flush().expect("Error flushing");
    stdin().read_line(&mut command).expect("Error reading");
    command.trim().to_owned()
}

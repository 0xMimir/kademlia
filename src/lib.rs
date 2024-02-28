#![feature(
    const_for,
    const_trait_impl,
    effects,
    const_mut_refs,
    iterator_try_collect
)]

#[macro_use]
extern crate serde;

#[macro_use]
extern crate log;

mod kademlia;
mod socket;
mod table;
mod types;

pub(crate) mod helpers;
mod pure;

pub use kademlia::Kademlia;
pub use types::key::Key;
pub use types::node::Node;

const KEY_SIZE: usize = 32;

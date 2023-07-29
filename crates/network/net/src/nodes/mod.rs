/*
   Appellation: nodes <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{client::*, cmds::*, node::*, queue::*};

mod client;
mod cmds;
mod node;
mod queue;

use crate::conduct::mainnet::Mainnet;
use crate::prelude::{FromPeer, NetworkEvent, Peer};
use fluidity_core::prelude::Power;
use tokio::sync::{mpsc, watch};

fn starter(
    capacity: Option<usize>,
    peer: Option<Peer>,
    power: Option<Power>,
) -> (NetworkClient, NetworkNode, watch::Sender<Power>) {
    let buffer: usize = capacity.unwrap_or(1024);
    let (cmds_tx, cmds_rx) = mpsc::channel::<NetworkCommand>(buffer);
    let (events_tx, events_rx) = mpsc::channel::<NetworkEvent>(buffer);
    let (power_tx, power_rx) = watch::channel(power.unwrap_or_default());
    let swarm = libp2p::Swarm::from_peer(peer.unwrap_or_default());
    let client = NetworkClient::new(cmds_tx);
    let node = NetworkNode::new(cmds_rx, events_tx, power_rx, swarm);
    (client, node, power_tx)
}

pub struct NetworkBuilder {
    capacity: Option<usize>,
    peer: Option<Peer>,
    power: Option<Power>,
    swarm: Option<libp2p::Swarm<Mainnet>>,
}

impl NetworkBuilder {
    pub fn new() -> Self {
        Self {
            capacity: None,
            peer: None,
            power: None,
            swarm: None,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity.unwrap_or(1024)
    }

    pub fn channels<T>(&self) -> (mpsc::Sender<T>, mpsc::Receiver<T>) {
        mpsc::channel::<T>(self.capacity())
    }

    pub fn watch(&self) -> (watch::Sender<Power>, watch::Receiver<Power>) {
        watch::channel(self.power.unwrap_or_default())
    }

    pub fn with_capacity(mut self, capacity: Option<usize>) -> Self {
        self.capacity = capacity;
        self
    }

    pub fn with_peer(mut self, peer: Option<Peer>) -> Self {
        self.peer = peer.clone();
        if let Some(p) = peer {
            self.swarm = Some(libp2p::Swarm::from_peer(p));
        }
        self
    }

    pub fn with_power(mut self, power: Option<Power>) -> Self {
        self.power = power;

        self
    }

    pub fn build(self) -> (NetworkClient, NetworkNode) {
        let (cmds_tx, cmds_rx) = mpsc::channel::<NetworkCommand>(self.capacity());
        let (events_tx, _events_rx) = mpsc::channel::<NetworkEvent>(self.capacity());
        let (power_tx, power_rx) = watch::channel(self.power.unwrap_or_default());
        let client = NetworkClient::new(cmds_tx);
        let node = NetworkNode::new(cmds_rx, events_tx, power_rx, self.swarm.unwrap());
        (client, node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let peer = Peer::default();
        let (client, node) = NetworkBuilder::new().with_peer(Some(peer)).build();
    }
}

/*
   Appellation: nodes <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{client::*, layer::*, node::*, queue::*};

mod client;
mod layer;
mod node;
mod queue;

use crate::prelude::{FromPeer, NetworkEvent, Peer};
use fluidity_core::prelude::Power;
use tokio::sync::{mpsc, watch};

pub struct NetworkStarter {
   client: NetworkClient,
   node: NetworkNode
}

impl NetworkStarter {
   pub fn new(capacity: Option<usize>, peer: Option<Peer>, power: watch::Receiver<Power>) -> Self {
      let buffer: usize = capacity.unwrap_or(1024);
      let (cmds_tx, cmds_rx) = mpsc::channel::<Command>(buffer);
      let (events_tx, events_rx) = mpsc::channel::<NetworkEvent>(buffer);
      let swarm = libp2p::Swarm::from_peer(peer.unwrap_or_default());
      let client = NetworkClient::new(cmds_tx);
      let node = NetworkNode::new(cmds_rx, events_tx, power, swarm);
      Self { client, node }
   }

}

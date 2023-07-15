/*
   Appellation: conduct <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Conduct
///
/// This module implements the behaviours of the
pub mod clusters;
pub mod mainnet;

use crate::peers::FromPeer;
use libp2p::swarm::NetworkBehaviour;

pub trait Conduct: FromPeer + NetworkBehaviour {}

impl<T> Conduct for T where T: FromPeer + NetworkBehaviour {}

pub enum Network {
    Mainnet,
}

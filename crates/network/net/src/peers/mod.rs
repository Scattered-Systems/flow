/*
    Appellation: peers <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Peers
///
///
pub use self::peer::*;

mod peer;

use crate::conduct::Conduct;
use libp2p::swarm::{Swarm, SwarmBuilder};

#[macro_export]
macro_rules! peer {
    () => {
        Peer::default()
    };
    ($seed: expr) => {
        Peer::try_from($seed).unwrap()
    };
}

#[macro_export]
macro_rules! swarm {
    ($peer: expr) => {
        Swarm::from($peer)
    };
}

/// [FromPeer] describes the conversion of a peer into some type
pub trait FromPeer {
    fn from_peer(peer: Peer) -> Self;
}

impl<T> FromPeer for T
where
    T: From<Peer>,
{
    fn from_peer(peer: Peer) -> Self {
        Self::from(peer)
    }
}

impl<B> FromPeer for Swarm<B>
where
    B: Conduct,
{
    fn from_peer(peer: Peer) -> Self {
        let behaviour = B::from_peer(peer.clone());
        SwarmBuilder::with_tokio_executor(peer.transport(), behaviour, peer.pid()).build()
    }
}

/// [IntoPeer] describes the conversion of some type into a peer
pub trait IntoPeer {
    fn into_peer(self) -> Peer;
}

impl<T> IntoPeer for T
where
    T: Into<Peer>,
{
    fn into_peer(self) -> Peer {
        self.into()
    }
}

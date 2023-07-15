use super::MainnetEvent;
use crate::{minis, peers::*, Conduct};
use libp2p::kad::{record::store::MemoryStore, Kademlia};
use libp2p::swarm::NetworkBehaviour;
use libp2p::{identify, identity::Keypair, mdns, ping};

/// [Subnet] describes the behaviour of a user owned cluster of nodes.
#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "MainnetEvent")]
pub struct Mainnet {
    pub identify: identify::Behaviour,
    pub kademlia: Kademlia<MemoryStore>,
    pub mdns: mdns::tokio::Behaviour,
    pub ping: ping::Behaviour,
    pub reqres: minis::reqres::ProtoBehaviour,
}

impl Mainnet {
    pub fn new(peer: Peer) -> Self {
        let pid = peer.pid();
        let identify = identify::Behaviour::new(identify::Config::new(
            "/flow/id/0.0.1".to_string(),
            peer.pk(),
        ));
        let kademlia = Kademlia::new(pid, MemoryStore::new(pid));
        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), pid).unwrap();
        Self {
            identify,
            kademlia,
            mdns,
            ping: Default::default(),
            reqres: minis::reqres::new(),
        }
    }
}

impl Default for Mainnet {
    fn default() -> Self {
        Self::from_peer(Peer::default())
    }
}

impl Conduct for Mainnet {}

impl FromPeer for Mainnet {
    fn from_peer(peer: Peer) -> Self {
        Self::new(peer)
    }
}

impl From<Keypair> for Mainnet {
    fn from(kp: Keypair) -> Self {
        Self::from_peer(Peer::new(kp))
    }
}

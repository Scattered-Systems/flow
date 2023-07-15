/*
   Appellation: mainnet <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::minis::{self, reqres::RequestResponseEvent};
/// # Mainnet
///
/// The mainnet describes the virtual networking layer
use crate::peers::Peer;
use libp2p::identity::Keypair;
use libp2p::kad::{self, record::store::MemoryStore};
use libp2p::request_response::Event as ReqResEvent;
use libp2p::swarm::NetworkBehaviour;
use libp2p::{identify, mdns, ping};

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "MainnetEvent")]
pub struct Mainnet {
    pub identify: identify::Behaviour,
    pub kademlia: kad::Kademlia<MemoryStore>,
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
        let kademlia = kad::Kademlia::new(pid, MemoryStore::new(pid));
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
        Self::new(Peer::default())
    }
}

impl From<Peer> for Mainnet {
    fn from(peer: Peer) -> Self {
        Self::new(peer)
    }
}

impl From<Keypair> for Mainnet {
    fn from(kp: Keypair) -> Self {
        Self::from(Peer::new(kp))
    }
}

/// [MainnetEvent] describes the events considered by the network
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum MainnetEvent {
    Identify(identify::Event),
    Kademlia(kad::KademliaEvent),
    Mdns(mdns::Event),
    Ping(ping::Event),
    RequestResponse(RequestResponseEvent),
}

impl From<identify::Event> for MainnetEvent {
    fn from(event: identify::Event) -> Self {
        Self::Identify(event)
    }
}

impl From<kad::KademliaEvent> for MainnetEvent {
    fn from(event: kad::KademliaEvent) -> Self {
        Self::Kademlia(event)
    }
}

impl From<mdns::Event> for MainnetEvent {
    fn from(event: mdns::Event) -> Self {
        Self::Mdns(event)
    }
}

impl From<ping::Event> for MainnetEvent {
    fn from(event: ping::Event) -> Self {
        Self::Ping(event)
    }
}

impl From<RequestResponseEvent> for MainnetEvent {
    fn from(event: RequestResponseEvent) -> Self {
        Self::RequestResponse(event)
    }
}

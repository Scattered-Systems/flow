/*
    Appellation: events <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Custom Network Events
*/
use crate::minis::reqres::ReqResEvent;
use libp2p::{identify, kad, mdns, ping};

/// [MainnetEvent] describes the events considered by the network
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum MainnetEvent {
    Identify(identify::Event),
    Kademlia(kad::KademliaEvent),
    Mdns(mdns::Event),
    Ping(ping::Event),
    RequestResponse(ReqResEvent),
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

impl From<ReqResEvent> for MainnetEvent {
    fn from(event: ReqResEvent) -> Self {
        Self::RequestResponse(event)
    }
}

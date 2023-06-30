/*
   Appellation: layer <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::proto::reqres::Response;
/// # Node Layer
///
/// This module implements the node layer; describing the available async operations
use crate::NetworkResult;
use libp2p::core::transport::ListenerId;
use libp2p::request_response::ResponseChannel;
use libp2p::{Multiaddr, PeerId};
use std::collections::HashSet;
use tokio::sync::oneshot;

pub type Commander<T = ()> = oneshot::Sender<NetworkResult<T>>;

#[derive(Debug)]
pub enum Command {
    Listen {
        addr: Multiaddr,
        tx: Commander<ListenerId>,
    },
    Dial {
        addr: Multiaddr,
        pid: PeerId,
        tx: Commander,
    },
    Provide {
        cid: String,
        tx: Commander,
    },
    Providers {
        cid: String,
        tx: Commander<HashSet<PeerId>>,
    },
    Request {
        payload: String,
        peer: PeerId,
        tx: Commander<Response>,
    },
    Respond {
        payload: Vec<u8>,
        channel: ResponseChannel<Response>,
    },
}

impl Command {
    pub fn dial(addr: Multiaddr, pid: PeerId, tx: Commander) -> Self {
        Self::Dial { addr, pid, tx }
    }
    pub fn listen(addr: Multiaddr, tx: Commander<ListenerId>) -> Self {
        Self::Listen { addr, tx }
    }
    pub fn provide(cid: String, tx: Commander) -> Self {
        Self::Provide { cid, tx }
    }
    pub fn providers(cid: String, tx: Commander<HashSet<PeerId>>) -> Self {
        Self::Providers { cid, tx }
    }
    pub fn request(payload: String, peer: PeerId, tx: Commander<Response>) -> Self {
        Self::Request { payload, peer, tx }
    }
    pub fn response(payload: Vec<u8>, channel: ResponseChannel<Response>) -> Self {
        Self::Respond { payload, channel }
    }
}

/*
   Appellation: layer <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::minis::reqres::Response;
/// # Node Layer
///
/// This module implements the node layer; describing the available async operations
use crate::NetworkResult;
use libp2p::core::transport::ListenerId;
use libp2p::request_response::ResponseChannel;
use libp2p::{Multiaddr, PeerId};
use std::collections::HashSet;
use tokio::sync::oneshot;

pub type NetworkCommandTx<T = ()> = oneshot::Sender<NetworkResult<T>>;

#[derive(Debug)]
pub enum NetworkCommand {
    Listen {
        addr: Multiaddr,
        tx: NetworkCommandTx<ListenerId>,
    },
    Dial {
        addr: Multiaddr,
        pid: PeerId,
        tx: NetworkCommandTx,
    },
    Provide {
        cid: String,
        tx: NetworkCommandTx,
    },
    Providers {
        cid: String,
        tx: NetworkCommandTx<HashSet<PeerId>>,
    },
    Request {
        payload: String,
        peer: PeerId,
        tx: NetworkCommandTx<Response>,
    },
    Respond {
        payload: Vec<u8>,
        channel: ResponseChannel<Response>,
    },
}

impl NetworkCommand {
    pub fn dial(addr: Multiaddr, pid: PeerId, tx: NetworkCommandTx) -> Self {
        Self::Dial { addr, pid, tx }
    }
    pub fn listen(addr: Multiaddr, tx: NetworkCommandTx<ListenerId>) -> Self {
        Self::Listen { addr, tx }
    }
    pub fn provide(cid: String, tx: NetworkCommandTx) -> Self {
        Self::Provide { cid, tx }
    }
    pub fn providers(cid: String, tx: NetworkCommandTx<HashSet<PeerId>>) -> Self {
        Self::Providers { cid, tx }
    }
    pub fn request(payload: String, peer: PeerId, tx: NetworkCommandTx<Response>) -> Self {
        Self::Request { payload, peer, tx }
    }
    pub fn response(payload: Vec<u8>, channel: ResponseChannel<Response>) -> Self {
        Self::Respond { payload, channel }
    }
}

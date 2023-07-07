/*
    Appellation: queue <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::NetworkCommandTx;
use crate::proto::reqres::{self, RequestId};
use libp2p::{kad::QueryId, PeerId};
use std::collections::{HashMap, HashSet};

/// The queue is a collection of all the pending requests.
#[derive(Debug, Default)]
pub struct Queue {
    pub dial: HashMap<PeerId, NetworkCommandTx>,
    pub start_providing: HashMap<QueryId, NetworkCommandTx>,
    pub get_providers: HashMap<QueryId, NetworkCommandTx<HashSet<PeerId>>>,
    pub requests: HashMap<RequestId, NetworkCommandTx<reqres::Response>>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            dial: HashMap::new(),
            start_providing: HashMap::new(),
            get_providers: HashMap::new(),
            requests: HashMap::new(),
        }
    }
}

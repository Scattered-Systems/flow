/*
   Appellation: client <nodes>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Command;
use crate::{proto::reqres::Response, NetworkResult};
use libp2p::core::transport::ListenerId;
use libp2p::request_response::ResponseChannel;
use libp2p::{Multiaddr, PeerId};
use std::collections::HashSet;
use tokio::sync::{mpsc, oneshot};

#[derive(Clone)]
pub struct NetworkClient {
    pub sender: mpsc::Sender<Command>,
}

impl NetworkClient {
    pub fn new(sender: mpsc::Sender<Command>) -> Self {
        Self { sender }
    }
    pub fn sender(&self) -> mpsc::Sender<Command> {
        self.sender.clone()
    }
    pub fn try_send(&mut self, command: Command) -> anyhow::Result<()> {
        self.sender.try_send(command)?;
        Ok(())
    }
    /// Dial the given peer at the given address.
    pub async fn dial(&mut self, addr: Multiaddr, pid: PeerId) -> NetworkResult {
        tracing::info!("Dialing {} at {}", pid, addr);
        let (tx, rx) = oneshot::channel();
        self.sender().send(Command::dial(addr, pid, tx)).await?;
        rx.await?
    }
    /// Listen for incoming connections on the given address.
    pub async fn listen(&mut self, addr: Multiaddr) -> NetworkResult<ListenerId> {
        let (tx, rx) = oneshot::channel();
        tracing::info!("Listening for incoming connections on {}", addr);
        self.sender().send(Command::listen(addr, tx)).await?;
        rx.await?
    }
    /// Advertise the local node as the provider of the given file on the DHT.
    pub async fn provide(&mut self, cid: String) -> NetworkResult {
        let (tx, rx) = oneshot::channel();
        self.sender().send(Command::provide(cid, tx)).await?;
        rx.await?
    }
    /// Find the providers for the given file on the DHT.
    pub async fn providers(&mut self, cid: String) -> NetworkResult<HashSet<PeerId>> {
        let (tx, rx) = oneshot::channel();
        self.sender().send(Command::providers(cid, tx)).await?;
        rx.await?
    }
    /// Request the content of the given file from the given peer.
    pub async fn request(&mut self, payload: String, peer: PeerId) -> NetworkResult<Response> {
        let (tx, rx) = oneshot::channel();
        self.sender()
            .send(Command::Request { payload, peer, tx })
            .await?;
        rx.await?
    }
    /// Respond with the provided file content to the given request.
    pub async fn respond(&mut self, payload: Vec<u8>, channel: ResponseChannel<Response>) {
        self.sender()
            .send(Command::Respond { payload, channel })
            .await
            .expect("Command receiver not to be dropped.");
    }
}

/*
   Appellation: node <nodes>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{NetworkCommand, Queue};
use crate::prelude::{Mainnet, MainnetEvent, NetworkEvent};
use anyhow::Result;
use fluidity_core::prelude::Power;
use futures::StreamExt;
use libp2p::kad::{self, KademliaEvent, QueryResult};
use libp2p::{
    multiaddr::Protocol,
    request_response,
    swarm::{SwarmEvent, THandlerErr},
    Swarm,
};
use std::collections::hash_map::Entry;
use tokio::sync::{mpsc, watch};
use tracing::instrument;

#[async_trait::async_trait]
pub trait NodeSpec {
    fn swarm(&self) -> Swarm<Mainnet>;

    async fn run(mut self);
}

pub struct NetworkNode {
    cmds: mpsc::Receiver<NetworkCommand>,
    events: mpsc::Sender<NetworkEvent>,
    power: watch::Receiver<Power>,
    queue: Queue,
    swarm: Swarm<Mainnet>,
}

impl NetworkNode {
    pub fn new(
        cmds: mpsc::Receiver<NetworkCommand>,
        events: mpsc::Sender<NetworkEvent>,
        power: watch::Receiver<Power>,
        swarm: Swarm<Mainnet>,
    ) -> Self {
        Self {
            cmds,
            events,
            power,
            queue: Queue::default(),
            swarm,
        }
    }
    #[instrument(skip(self), name = "handle::command", target = "node")]
    async fn handle_command(&mut self, cmd: NetworkCommand) -> Result<()> {
        match cmd {
            NetworkCommand::Dial { addr, pid, tx } => match self.queue.dial.entry(pid) {
                Entry::Occupied(_) => {
                    tracing::warn!("The peer ({}) is already being dialed", pid);
                }
                Entry::Vacant(entry) => {
                    self.swarm
                        .behaviour_mut()
                        .kademlia
                        .add_address(&pid, addr.clone());
                    let dialopts = addr.with(Protocol::P2p((pid).into()));
                    match self.swarm.dial(dialopts) {
                        Err(e) => {
                            let _ = tx.send(Err(e.into()));
                        }
                        Ok(_) => {
                            entry.insert(tx);
                        }
                    }
                }
            },
            NetworkCommand::Listen { addr, tx } => {
                let msg = self.swarm.listen_on(addr).map_err(|e| e.into());
                tracing::info!("Listening on {:?}", msg);
                tx.send(msg).expect("Receiver to be still open.");
            }
            _ => todo!(),
        }
        Ok(())
    }

    /// Handle events from the swarm; the stateful network manager
    #[instrument(skip(self), name = "handle::event", target = "node")]
    async fn handle_event(
        &mut self,
        event: SwarmEvent<MainnetEvent, THandlerErr<Mainnet>>,
    ) -> Result<()> {
        match event {
            // Handle custom networking events
            SwarmEvent::Behaviour(mainnet) => match mainnet {
                MainnetEvent::Identify(identify) => match identify {
                    libp2p::identify::Event::Received { peer_id, .. } => {
                        tracing::info!("Identified peer: {}", peer_id);
                    }
                    e => tracing::warn!("Unhandled identify event: {:?}", e),
                },
                MainnetEvent::Kademlia(kademlia) => match kademlia {
                    KademliaEvent::OutboundQueryProgressed { id, result, .. } => match result {
                        QueryResult::GetProviders(Ok(get_providers)) => match get_providers {
                            kad::GetProvidersOk::FoundProviders { providers, .. } => {
                                if let Some(tx) = self.queue.get_providers.remove(&id) {
                                    tx.send(Ok(providers)).expect("Receiver not to be dropped");
                                    // Finish the query. We are only interested in the first result.
                                    self.swarm
                                        .behaviour_mut()
                                        .kademlia
                                        .query_mut(&id)
                                        .unwrap()
                                        .finish();
                                }
                            }
                            e => tracing::warn!("Unhandled get providers result: {:?}", e),
                        },
                        QueryResult::StartProviding(_) => {
                            let sender = self
                                .queue
                                .start_providing
                                .remove(&id)
                                .expect("Completed query to be previously pending.");
                            let _ = sender.send(Ok(()));
                        }
                        e => tracing::warn!("Unhandled query result: {:?}", e),
                    },
                    _ => {}
                },
                MainnetEvent::Mdns(mdns) => match mdns {
                    libp2p::mdns::Event::Discovered(disc) => {
                        for (pid, addr) in disc {
                            tracing::info!("Discovered peer: {} at {}", pid, addr);
                        }
                    }
                    e => tracing::warn!("Unhandled mdns event: {:?}", e),
                },
                MainnetEvent::RequestResponse(reqres) => match reqres {
                    request_response::Event::Message { message, .. } => match message {
                        request_response::Message::Request {
                            request, channel, ..
                        } => {
                            self.events
                                .send(NetworkEvent::inbound_request(request, channel))
                                .await
                                .expect("Receiver not to be dropped");
                        }
                        request_response::Message::Response {
                            response,
                            request_id,
                        } => {
                            let _ = self
                                .queue
                                .requests
                                .remove(&request_id)
                                .expect("pending...")
                                .send(Ok(response));
                        }
                    },
                    request_response::Event::OutboundFailure {
                        request_id, error, ..
                    } => {
                        let _ = self
                            .queue
                            .requests
                            .remove(&request_id)
                            .expect("pending...")
                            .send(Err(error.into()));
                    }
                    request_response::Event::InboundFailure {
                        request_id, error, ..
                    } => {
                        let _ = self
                            .queue
                            .requests
                            .remove(&request_id)
                            .expect("pending...")
                            .send(Err(error.into()));
                    }
                    request_response::Event::ResponseSent { .. } => todo!(),
                },
                e => tracing::warn!("Unhandled subnet event: {:?}", e),
            },
            SwarmEvent::ConnectionEstablished {
                peer_id, endpoint, ..
            } => {
                if let libp2p::core::ConnectedPoint::Dialer { .. } = endpoint {
                    if let Some(tx) = self.queue.dial.remove(&peer_id) {
                        tx.send(Ok(())).expect("Receiver not to be dropped");
                    }
                }
            }
            SwarmEvent::Dialing {
                peer_id,
                connection_id: _,
            } => {
                if let Some(tx) = self.queue.dial.remove(&peer_id.expect("")) {
                    tx.send(Ok(())).expect("Receiver not to be dropped");
                }
                tracing::info!("Dialing peer: {}", peer_id.expect(""));
            }
            SwarmEvent::NewListenAddr { address, .. } => {
                let pid = *self.swarm.local_peer_id();
                tracing::info!(
                    "Local node is listening on {:?}",
                    address.with(Protocol::P2p(pid.into()))
                );
            }
            e => tracing::warn!("Unhandled swarm event: {:?}", e),
        };
        Ok(())
    }
    #[instrument(skip(self), name = "run", target = "node")]
    pub async fn run(mut self) {
        loop {
            tokio::select! {
                Some(cmd) = self.cmds.recv() => {
                    self.handle_command(cmd).await.expect("Command Error");
                }
                Some(event) = self.swarm.next() => {
                    self.handle_event(event).await.expect("Event Error");
                }
                _ = tokio::signal::ctrl_c() => {
                    tracing::info!("Shutting down...");
                    break;
                }
                Ok(_) = self.power.changed() => {
                    match *self.power.borrow() {
                        Power::Off => {
                            tracing::info!("Node: shutting down...");
                            break;
                        }
                        Power::On => {
                            tracing::info!("Node: powering on...");
                        }
                    }
                }

            }
        }
    }

    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(self.run())
    }
}

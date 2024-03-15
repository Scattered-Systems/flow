/*
    Appellation: platform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::context::PlatformContext;
use crate::prelude::Power;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, watch};
use tracing::instrument;

pub enum PlatformMode {
    AccessPoint,
    Node,
}

pub enum PlatformEvent {
    Workload(WorkloadEvent),
}

pub enum WorkloadEvent {
    Register { id: String },
}

pub struct Platform {
    ctx: Arc<PlatformContext>,
    mode: PlatformMode,
    power: watch::Sender<Power>,
    state: String,
    tasks: mpsc::Receiver<String>,
}

impl Platform {
    #[instrument(name = "run", skip(self), target = "platform")]
    pub async fn run(self) {
        loop {
            tokio::select! {

                _ = tokio::signal::ctrl_c() => {
                    let _ = self.power.send(Power::Off).expect("Power Error");

                    tracing::info!("Shutting down...");
                    break;
                }
            }
        }
    }
}

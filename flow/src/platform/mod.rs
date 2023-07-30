/*
   Appellation: platform <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Platform
//! 
//! The platform module is responsible for managing the platform's state and data-layer
pub use self::{cmds::*, events::*, frame::*};

mod cmds;
mod events;
mod frame;

use crate::Context;
use fluidity::prelude::Power;
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, watch};

pub struct PlatformEngine {
    commands: mpsc::Receiver<PlatformArgs>,
    events: mpsc::Sender<PlatformEvent>,
    power: watch::Receiver<Power>,
}

impl PlatformEngine {
    pub fn new(
        commands: mpsc::Receiver<PlatformArgs>,
        events: mpsc::Sender<PlatformEvent>,
        power: watch::Receiver<Power>,
    ) -> Self {
        Self {
            commands,
            events,
            power,
        }
    }

    #[instrument(skip(self), name = "handle", target = "executor")]
    async fn handle_command(&mut self, args: &PlatformArgs) -> AsyncResult<()> {
        if let Some(cmd) = &args.command  {
            match cmd.clone() {
                PlatformCommand::Connect { target } => {
                    tracing::info!("Connecting to {}", target.unwrap_or_default());
                }
            }
        }
        Ok(())
    }
    #[instrument(skip(self), name = "run", target = "executor")]
    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                Some(args) = self.commands.recv() => {
                    self.handle_command(&args).await;
                }
                Ok(_) = self.power.changed() => {
                    match self.power.borrow().clone() {
                        Power::On => tracing::info!("Power on"),
                        Power::Off => {
                            tracing::info!("Power off");
                            break;
                        },
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    tracing::info!("Ctrl-C received");
                    break;
                }
            }
        }
    }

    pub async fn spawn(mut self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            self.run().await;
        })
    }
}



pub struct Platform {
    context: Arc<Mutex<Context>>,
    engine: Arc<PlatformEngine>,
}

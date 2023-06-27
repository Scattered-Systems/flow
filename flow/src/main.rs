/*
   Appellation: flow <binary>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::{context::*, settings::*, states::*};

mod context;
mod settings;
mod states;

pub mod events;

#[cfg(not(target_family = "wasm32-wasi"))]
use tokio::sync::{mpsc, watch};
#[cfg(target_family = "wasm32-wasi")]
use tokio_wasi::sync::{mpsc, watch};

use events::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    
    starter().run().await?;

    Ok(())
}

pub fn starter() -> Flow {
    let (events_tx, events_rx) = mpsc::unbounded_channel::<FlowEvent>();
    let (power_tx, power_rx) = watch::channel::<PowerEvent>(Default::default());

    let settings = Settings::new(None);
    
    return Flow::new(events_rx, power_tx, settings).init();
}

pub struct Flow {
    context: Context,
    events: mpsc::UnboundedReceiver<FlowEvent>,
    power: watch::Sender<PowerEvent>,
}

impl Flow {
    pub fn new(
        events: mpsc::UnboundedReceiver<FlowEvent>,
        power: watch::Sender<PowerEvent>,
        settings: Settings,
    ) -> Self {
        Self {
            context: Context::new(settings.clone()),
            events,
            power,
        }
    }
    pub fn init(self) -> Self {
        self.context.init_tracing();
        tracing::info!("Flow: Initializing systems...");
        self
    }
}

impl EventHandle for Flow {
    fn handle_event(&self, event: FlowEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match event {
            events::FlowEvent::Power(power) => {
                tracing::info!("{:?}", power);
            }
        }
        Ok(())
    }
}

#[cfg(not(target_family = "wasm32-wasi"))]
impl Flow {
    pub async fn run(mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(loop {
            tokio::select! {
                event = self.events.recv() => {
                    if let Some(event) = event {
                        EventHandle::handle_event(&self, event).expect("Event Error");
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    let _ = self.power.send(events::PowerEvent::Off);
                    tracing::info!("Shutting down...");
                    break;
                }
            }
        })
    }
}
#[cfg(target_family = "wasm32-wasi")]
impl Flow {
    pub async fn run(mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(loop {
            tokio_wasi::select! {
                event = self.events.recv() => {
                    if let Some(event) = event {
                        EventHandle::handle_event(&self, event).expect("Event Error");
                    }
                }
                _ = tokio_wasi::signal::ctrl_c() => {
                    let _ = self.power.send(PowerEvent::Off);
                    tracing::info!("Shutting down...");
                    break;
                }
            }
        })
    }
}

pub struct FlowNode {}

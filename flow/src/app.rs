/*
   Appellation: app <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::events::*;
use crate::{Context, Settings};
use anyhow::Result;
#[cfg(not(feature = "wasm"))]
use tokio::sync::{mpsc, watch};
#[cfg(feature = "wasi")]
use tokio_wasi::sync::{mpsc, watch};

pub fn starter() -> Flow {
    let (_, events_rx) = mpsc::unbounded_channel::<FlowEvent>();
    let (power_tx, _) = watch::channel::<PowerEvent>(Default::default());

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

impl EventHandler for Flow {
    fn handle_event(&self, event: FlowEvent) -> Result<()> {
        match event {
            FlowEvent::Power(power) => {
                tracing::info!("{:?}", power);
            }
        }
        Ok(())
    }
}

#[cfg(not(target_family = "wasm32-wasi"))]
impl Flow {
    pub async fn run(mut self) -> Result<()> {
        Ok(loop {
            tokio::select! {
                event = self.events.recv() => {
                    if let Some(event) = event {
                        EventHandler::handle_event(&self, event).expect("Event Error");
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    let _ = self.power.send(PowerEvent::Off);
                    tracing::info!("Shutting down...");
                    break;
                }
            }
        })
    }
}
#[cfg(target_family = "wasm32-wasi")]
impl Flow {
    pub async fn run(mut self) -> Result<()> {
        Ok(loop {
            tokio_wasi::select! {
                event = self.events.recv() => {
                    if let Some(event) = event {
                        EventHandler::handle_event(&self, event).expect("Event Error");
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

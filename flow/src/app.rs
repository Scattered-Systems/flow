/*
   Appellation: app <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::events::FlowEvent;
use crate::{Context, Settings};
use fluidity::prelude::{AsyncResult, Power, EventHandle};

#[cfg(any(target_family = "unix", target_family = "windows"))]
#[cfg(not(any(feature = "wasi", feature = "wasm", target_family = "wasm", target_os = "wasi")))]
use tokio::sync::{mpsc, watch};
#[cfg(any(feature = "wasi", target_os = "wasi"))]
use tokio_wasi::sync::{mpsc, watch};

pub fn starter() -> Flow {
    let (_, events_rx) = mpsc::unbounded_channel::<FlowEvent>();
    let (power_tx, _) = watch::channel::<Power>(Default::default());

    let settings = Settings::new(None);

    return Flow::new(events_rx, power_tx, settings).init();
}

pub struct Flow {
    context: Context,
    events: mpsc::UnboundedReceiver<FlowEvent>,
    
    power: watch::Sender<Power>,
}

impl Flow {
    pub fn new(
        events: mpsc::UnboundedReceiver<FlowEvent>,
        power: watch::Sender<Power>,
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

impl EventHandle<FlowEvent> for Flow {
    fn handle_event(&self, event: FlowEvent) -> AsyncResult<()> {
        match event {
            _ => {
                tracing::warn!("Unhandled Event: {:?}", event);
            }
        }
        Ok(())
    }
}

#[cfg(any(target_family = "unix", target_family = "windows"))]
#[cfg(not(any(feature = "wasi", feature = "wasm", target_family = "wasm", target_os = "wasi")))]
impl Flow {
    pub async fn run(mut self) -> anyhow::Result<()> {
        Ok(loop {
            tokio::select! {
                event = self.events.recv() => {
                    if let Some(event) = event {
                        EventHandle::handle_event(&self, event).expect("Event Error");
                    }
                }
                
                _ = tokio::signal::ctrl_c() => {
                    let _ = self.power.send(Power::Off);
                    tracing::info!("Shutting down...");
                    break;
                }
            }
        })
    }
}

#[cfg(any(feature = "wasi", target_os = "wasi"))]
impl Flow {
    pub async fn run(mut self) -> anyhow::Result<()> {
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

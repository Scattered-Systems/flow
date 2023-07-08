/*
   Appellation: app <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::events::FlowEvent;
use crate::platform::PlatformCommand;
use crate::{Context, Settings};
use fluidity::prelude::{AsyncResult, EventHandle, Power};

#[cfg(any(target_family = "unix", target_family = "windows"))]
#[cfg(not(any(
    feature = "wasi",
    feature = "wasm",
    target_family = "wasm",
    target_os = "wasi"
)))]
use tokio::sync::{mpsc, watch};
#[cfg(any(feature = "wasi", target_os = "wasi"))]
use tokio_wasi::sync::{mpsc, watch};

pub fn starter() -> Flow {
    let buffer: usize = 12;
    let mut args = std::env::args_os();
    let _ = args.next().expect("No args");
    let (_, io_rx) = watch::channel::<std::env::ArgsOs>(args);
    let (commands_tx, commands_rx) = mpsc::channel::<PlatformCommand>(buffer);
    let (_, events_rx) = mpsc::channel::<FlowEvent>(buffer);

    let (power_tx, _) = watch::channel::<Power>(Default::default());

    let settings = Settings::new(None);
    let client = FlowClient::new(commands_tx);
    return Flow::new(commands_rx, events_rx, power_tx, settings).init();
}

pub struct FlowClient {
    pub commands: mpsc::Sender<PlatformCommand>,
}

impl FlowClient {
    pub fn new(commands: mpsc::Sender<PlatformCommand>) -> Self {
        Self { commands }
    }
}

pub struct Flow {
    context: Context,
    commands: mpsc::Receiver<PlatformCommand>,
    
    events: mpsc::Receiver<FlowEvent>,
    power: watch::Sender<Power>,
}

impl Flow {
    pub fn new(
        commands: mpsc::Receiver<PlatformCommand>,
        events: mpsc::Receiver<FlowEvent>,
        power: watch::Sender<Power>,
        settings: Settings,
    ) -> Self {
        Self {
            context: Context::new(settings.clone()),
            commands,
            events,
            power,
        }
    }
    pub fn init(self) -> Self {
        self.context.init_tracing();
        tracing::info!("Flow: Initializing systems...");
        self
    }

    pub fn handle_command(&self, command: PlatformCommand) -> AsyncResult<()> {
        match command {
            _ => {
                tracing::warn!("Unhandled Command: {:?}", command);
            }
        }
        Ok(())
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
#[cfg(not(any(
    feature = "wasi",
    feature = "wasm",
    target_family = "wasm",
    target_os = "wasi"
)))]
impl Flow {
    pub async fn run(mut self) -> anyhow::Result<()> {
        Ok(loop {
            tokio::select! {
                Some(command) = self.commands.recv() => {
                    self.handle_command(command).expect("Command Error");
                }
                Some(event) = self.events.recv() => {
                    EventHandle::handle_event(&self, event).expect("Event Error");
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

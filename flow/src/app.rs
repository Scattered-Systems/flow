/*
   Appellation: app <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Flow
/// 
/// The platform agnostic core of the Flow network.
use crate::events::FlowEvent;
use crate::platform::{client::FlowClient, PlatformCommand};
use crate::{Context, Settings};
use fluidity::prelude::{AsyncResult, EventHandle, Power, State};
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, watch};
use tokio::task::JoinHandle;


pub struct Flow {
    context: Arc<Mutex<Context>>,
    commands: mpsc::Receiver<PlatformCommand>,
    events: mpsc::Sender<FlowEvent>,
    state: Arc<Mutex<State>>,
    power: watch::Sender<Power>,
}

impl Flow {
    pub fn new(
        commands: mpsc::Receiver<PlatformCommand>,
        events: mpsc::Sender<FlowEvent>,
        power: watch::Sender<Power>,
        settings: Settings,
    ) -> Self {
        let context = Arc::new(Mutex::new(Context::new(settings)));
        let state = Arc::new(Mutex::new(State::default()));
        Self {
            context,
            commands,
            events,
            state,
            power,
        }
    }
    pub fn init(self) -> Self {
        self.context.lock().unwrap().init_tracing();
        tracing::info!("Initializing systems...");
        self
    }

    async fn handle_command(&mut self, command: &PlatformCommand) -> AsyncResult<()> {
        match command {
            _ => {
                self.events
                    .send(FlowEvent::Response {
                        message: "".to_string(),
                    })
                    .await
                    .expect("Event Error");
                tracing::warn!("Unhandled Command: {:?}", command);
            }
        }
        Ok(())
    }

    pub async fn run(mut self) -> () {
        loop {
            tokio::select! {
                Some(command) = self.commands.recv() => {
                    let mut state = self.state();
                    state.update(State::valid(&command));
                    self.state.lock().unwrap().update(state);
                    self.handle_command(&command).await.expect("Command Error");
                }
                _ = tokio::signal::ctrl_c() => {
                    let _ = self.power.send(Power::Off);
                    tracing::info!("Shutting down...");
                    break;
                }
            }
        }
    }

    pub fn spawn(self) -> JoinHandle<()> {
        tokio::spawn(self.run())
    }

    pub fn state(&self) -> State {
        self.state.lock().unwrap().clone()
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

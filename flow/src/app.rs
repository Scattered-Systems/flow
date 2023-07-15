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
use tracing::instrument;

pub struct Flow {
    context: Arc<Mutex<Context>>,
    commands: mpsc::Receiver<PlatformCommand>,
    events: mpsc::Sender<FlowEvent>,
    power: watch::Sender<Power>,
    state: Arc<Mutex<State>>,
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
            power,
            state,
        }
    }

    pub fn context(&self) -> Context {
        self.context.lock().unwrap().clone()
    }
    pub fn init(self) -> Self {
        self.context.lock().unwrap().init_tracing();
        tracing::info!("Initializing systems...");
        self
    }
    #[instrument(skip(self), name = "handler")]
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

    #[instrument(skip(self), name = "runner")]
    pub async fn run(mut self) {
        loop {
            tokio::select! {
                Some(command) = self.commands.recv() => {
                    let mut state = self.state();
                    state.set_message(&command);
                    self.state.lock().unwrap().set_message(&command);
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
    #[instrument(skip(self), name = "flow")]
    pub fn spawn(self) -> JoinHandle<()> {
        tokio::spawn(self.run())
    }

    pub fn state(&self) -> State {
        self.state.lock().unwrap().clone()
    }
}

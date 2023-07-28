/*
   Appellation: app <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::clients::FlowClient;
use crate::events::FlowEvent;
use crate::platform::PlatformCommand;
/// # Flow
///
/// The platform agnostic core of the Flow network.
use crate::{Context, Settings};
use fluidity::prelude::{AsyncResult, Power, State};
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, watch};
use tokio::task::JoinHandle;
use tracing::instrument;

pub struct Flow {
    context: Arc<Mutex<Context>>,
    commands: mpsc::Receiver<PlatformCommand>,
    events: mpsc::Sender<FlowEvent>,
    power: watch::Sender<Power>,
}

impl Flow {
    pub fn new(
        commands: mpsc::Receiver<PlatformCommand>,
        events: mpsc::Sender<FlowEvent>,
        power: watch::Sender<Power>,
        settings: Settings,
    ) -> Self {
        let context = Context::new(settings, State::default());
        Self {
            context: Arc::new(Mutex::new(context)),
            commands,
            events,
            power,
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
    #[instrument(fields(message = %command), skip(self), name = "handler", target = "flow")]
    async fn handle_command(&mut self, command: &PlatformCommand) -> AsyncResult<()> {
        match command {
            _ => {
                tracing::warn!("Unhandled Command");
            }
        }
        Ok(())
    }

    #[instrument(fields(state = %self.context().state().state()), skip(self), name = "run", target = "flow")]
    pub async fn run(mut self) {
        loop {
            tokio::select! {
                Some(command) = self.commands.recv() => {
                    self.context.lock().unwrap().state_mut().set_message(&command);
                    self.handle_command(&command).await.expect("Command Error");
                }
                _ = tokio::signal::ctrl_c() => {
                    let _ = self.power.send(Power::Off).expect("Power Error");

                    tracing::info!("Shutting down...");
                    break;
                }
            }
        }
    }
    #[instrument(skip(self), name = "spawn", target = "flow")]
    pub fn spawn(self) -> JoinHandle<()> {
        tokio::spawn(self.run())
    }

}

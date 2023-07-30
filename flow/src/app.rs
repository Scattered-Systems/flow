/*
   Appellation: app <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::events::FlowEvent;
use crate::platform::{PlatformArgs, PlatformCommand};
/// # Flow
///
/// The platform agnostic core of the Flow network.
use crate::{Context, Settings};
use fluidity::prelude::{AsyncResult, Power, State};
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, watch};
use tokio::task::JoinHandle;
use tracing::instrument;
use tracing_subscriber::util::SubscriberInitExt;

pub trait AppInitializer {
    fn init(self) -> Self;
    fn init_tracing(self) -> Self;
}

pub struct Flow {

    context: Arc<Mutex<Context>>,
    commands: mpsc::Receiver<PlatformArgs>,
    events: mpsc::Sender<FlowEvent>,
    power: watch::Sender<Power>,
}

impl Flow {
    pub fn new(
        commands: mpsc::Receiver<PlatformArgs>,
        events: mpsc::Sender<FlowEvent>,
        power: watch::Sender<Power>,
        settings: Settings,
    ) -> Self {
        let context = Context::new(settings, tokio::runtime::Handle::current(), State::default());
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
    #[instrument(skip(self), name = "init", target = "flow")]
    pub fn init(self) -> Self {
        tracing::info!("Initializing systems...");
        self
    }

    #[instrument(fields(message = %args), skip(self), name = "handler", target = "flow")]
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

    #[instrument(fields(state = %self.context().state().state()), skip(self), name = "run", target = "flow")]
    pub async fn run(mut self) {
        loop {
            tokio::select! {
                Some(command) = self.commands.recv() => {
                    self.context.lock().unwrap().state_mut().set_message(&command);
                    if let Err(e) = self.handle_command(&command).await {
                        tracing::error!("Command Error: {:?}", e);
                        self.context.lock().unwrap().state_mut().invalidate();
                    }
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

    pub fn with_tracing(self) -> Self {
        self.context().settings().logger().subscriber().init();
        self
    }
}

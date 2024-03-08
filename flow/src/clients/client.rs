/*
    Appellation: client <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::platform::PlatformCommand;
use tokio::sync::mpsc::{self, error::SendError};

#[derive(Clone, Debug)]
pub struct FlowClient {
    commands: mpsc::Sender<PlatformCommand>,
}

impl FlowClient {
    pub fn new(commands: mpsc::Sender<PlatformCommand>) -> Self {
        Self { commands }
    }
    pub fn commands(&self) -> mpsc::Sender<PlatformCommand> {
        self.commands.clone()
    }
    pub async fn send(&self, command: PlatformCommand) -> Result<(), SendError<PlatformCommand>> {
        self.commands.send(command).await
    }
}

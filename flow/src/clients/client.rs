/*
    Appellation: client <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::platform::{PlatformArgs, PlatformCommand};
use tokio::sync::mpsc::{self, error::SendError};

#[derive(Clone, Debug)]
pub struct FlowClient {
    pub commands: mpsc::Sender<PlatformArgs>,
}

impl FlowClient {
    pub fn new(commands: mpsc::Sender<PlatformArgs>) -> Self {
        Self { commands }
    }
    pub fn commands(&self) -> mpsc::Sender<PlatformArgs> {
        self.commands.clone()
    }
    pub async fn send(&self, command: PlatformArgs) -> Result<(), SendError<PlatformArgs>> {
        self.commands.send(command).await
    }
}

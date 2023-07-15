/*
    Appellation: client <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::PlatformCommand;
use tokio::sync::{mpsc, watch};

#[derive(Clone, Debug)]
pub struct FlowClient {
    pub commands: mpsc::Sender<PlatformCommand>,
}

impl FlowClient {
    pub fn new(commands: mpsc::Sender<PlatformCommand>) -> Self {
        Self { commands }
    }
}

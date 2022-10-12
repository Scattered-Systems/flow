/*
    Appellation: interface <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use super::states::State;
use crate::{api::Api, cli::CommandLineInterface, Context, Settings};
use scsys::core::{BoxResult, Error};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Application {
    pub context: Context,
    pub state: State,
}

impl Application {
    pub fn new(settings: Settings) -> Self {
        let context = Context::new(settings);
        let state = State::new("initializing");
        Self { context, state }
    }
    pub fn logging(&mut self) -> &Self {
        match &self.context.settings.logger {
            Some(v) => v.setup(),
            None => scsys::prelude::Logger::from("debug").setup(),
        }
        self
    }
    pub fn set_state(&mut self, state: State) -> &Self {
        self.state = state;
        self
    }
    pub async fn api(&self) -> BoxResult<&Self> {
        let api = Api::new(self.context.clone());
        api.run().await?;
        Ok(self)
    }
    pub fn cli(&self) -> BoxResult<CommandLineInterface> {
        Ok(CommandLineInterface::default())
    }
    pub async fn rpc(&self) -> BoxResult<&Self> {
        super::rpc::RPCBackend::new(self.context.clone()).run().await?;
        Ok(self)
    }
    pub async fn run(&self) -> BoxResult<&Self> {
        let _data = self.cli()?;
        Ok(self)
    }
}

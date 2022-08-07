/*
    Appellation: interface <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{api, cli, handlers::handle_cli_inputs};
use acme::prelude::{async_trait, APISpec, CLISpec};
use scsys::{BoxResult, Temporal};

#[derive(Clone, Copy, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
pub enum AppState {
    Off,
    On,
}

impl Default for AppState {
    fn default() -> Self {
        Self::Off
    }
}

#[derive(Clone, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
pub struct Flow {
    pub state: AppState,
    pub timestamp: i64,
}

impl Flow {
    fn constructor(state: AppState, timestamp: i64) -> Self {
        Self { state, timestamp }
    }
    pub fn new(state: AppState) -> Self {
        Self::constructor(state, Temporal::now().timestamp())
    }
}

#[async_trait]
impl APISpec<api::FlowAPI> for Flow {
    async fn run(&self) -> BoxResult<()>
        where
            Self: Sized + Sync,
    {
        let api = api::FlowAPI::default();
        println!("{}", &api);
        api.server()
            .await
            .ok()
            .unwrap()
            .await
            .expect("Server Error");
        Ok(())
    }
}

impl CLISpec<cli::FlowCLI> for Flow {
    fn run(&self) -> BoxResult<()>
        where
            Self: Sized,
    {
        let data = cli::FlowCLI::default();
        println!("{:#?}", data.action.clone());
        println!("{:#?}", data.command.clone());
        Ok(handle_cli_inputs(data))
    }
}

impl Default for Flow {
    fn default() -> Self {
        Self::new(AppState::default())
    }
}

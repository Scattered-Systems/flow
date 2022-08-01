/*
    Appellation: interface <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::api::FlowAPI;
use acme::network::{async_trait, APISpec};

#[derive(Clone, Copy, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum AppState {
    Off,
    On,
}

impl Default for AppState {
    fn default() -> Self {
        Self::Off
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Flow {
    pub state: AppState,
    pub timestamp: i64,
}

impl Flow {
    fn constructor(state: AppState, timestamp: i64) -> Self {
        Self { state, timestamp }
    }
    pub fn new(state: AppState) -> Self {
        Self::constructor(state, scsys::Utc::now().timestamp())
    }
}

#[async_trait]
impl APISpec<FlowAPI> for Flow {
    async fn run(&self) -> Result<(), scsys::BoxError>
        where
            Self: Sized + Sync,
    {
        let api = FlowAPI::default();
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

impl Default for Flow {
    fn default() -> Self {
        Self::new(AppState::default())
    }
}

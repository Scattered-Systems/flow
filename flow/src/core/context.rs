/*
   Appellation: context
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use super::Settings;
use scsys::prelude::{Cache, Database, Web3Provider};

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Context {
    pub settings: Option<Settings>,
}

impl Context {
    pub fn new(settings: Option<Settings>) -> Self {
        Self { settings }
    }
    pub fn settings(&self) -> Settings {
        self.settings.clone().expect(format!("{:?}", scsys::Error::default()).as_str())
    }
    pub fn set_settings(&mut self, settings: Settings) -> &Self {
        self.settings = Some(settings);
        self
    }
    pub fn cache(&self) -> Option<Cache> {
        self.settings().providers.cache.clone()
    }
    pub fn database(&self) -> Option<Database> {
        self.settings().providers.database.clone()
    }
    pub fn ethereum(&self) -> Option<Web3Provider> {
        self.settings().providers.ethereum.clone()
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.settings {
            Some(v) => write!(f, "Context({})", v),
            None => write!(f, "Context(None)")
        }
    }
}

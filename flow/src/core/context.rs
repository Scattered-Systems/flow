/*
   Appellation: context
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use super::Settings;
use scsys::core::{providers::{Cache, Database, Web3Provider}};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Context {
    pub settings: Option<Settings>,
}

impl Context {
    pub fn new(settings: Option<Settings>) -> Self {
        Self { settings }
    }
    pub fn settings(&self) -> Settings {
        self.settings.clone().expect(format!("{:?}", scsys::core::Error::default()).as_str())
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

impl scsys::core::Context<Settings> for Context {
    fn context(&self) -> Self {
        self.clone()
    }

    fn settings(&self) -> Settings {
        match self.settings.clone() {
            Some(v) => v,
            None => panic!("{:?}", scsys::core::Error::default())
        }
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

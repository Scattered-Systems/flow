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
        self.settings.expect(format!("{:?}", scsys::Error::default()).as_str())
    }
    pub fn set_settings(&mut self, settings: Settings) -> &Self {
        self.settings = Some(settings);
        self
    }
    pub fn cache(&self) -> Cache {
        self.settings().cache.clone()
    }
    pub fn database(&self) -> Database {
        self.settings().database.clone()
    }
    pub fn provider(&self) -> Web3Provider {
        self.settings().provider.clone()
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.settings {
            Some(v) => write!(f, "Context({})", v),
            None => write!(f, "Context(None)")
        }
    }
}

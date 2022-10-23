/*
   Appellation: context
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use super::Settings;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Context {
    pub settings: Settings,
}

impl Context {
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }
}

impl scsys::Context<Settings> for Context {
    fn context(&self) -> Self {
        self.clone()
    }

    fn settings(&self) -> Settings {
        self.settings.clone()
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.settings)
    }
}

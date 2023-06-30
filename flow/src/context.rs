/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::Settings;
use serde::{Deserialize, Serialize};

use tracing_subscriber::{fmt, EnvFilter};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Context {
    cnf: Settings,
}

impl Context {
    pub fn new(cnf: Settings) -> Self {
        Self { cnf }
    }
    pub fn init_tracing(&self) {
        self.settings().logger.setup_env();
        fmt::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
    }
    pub fn settings(&self) -> Settings {
        self.cnf.clone()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            cnf: Settings::default(),
        }
    }
}

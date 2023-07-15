/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
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
            .compact()
            .with_env_filter(EnvFilter::from_default_env())
            .with_line_number(false)
            .with_target(true)
            .with_thread_ids(false)
            .init();
    }
    pub fn settings(&self) -> Settings {
        self.cnf.clone()
    }
}

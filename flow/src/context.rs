/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Settings;
use fluidity::prelude::State;
use serde::{Deserialize, Serialize};

use tracing_subscriber::{fmt, EnvFilter};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Context {
    cnf: Settings,
    state: State,
}

impl Context {
    pub fn new(cnf: Settings, state: State) -> Self {
        Self { cnf, state }
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

    pub fn state(&self) -> State {
        self.state.clone()
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

}

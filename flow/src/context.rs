/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Settings;
use fluidity::prelude::State;
use serde::{Deserialize, Serialize};

use tracing_subscriber::{fmt, EnvFilter};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Context {
    cnf: Settings,
    state: State,
    
}

impl Context {
    pub fn new(cnf: Settings, state: State) -> Self {
        Self { cnf, state }
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

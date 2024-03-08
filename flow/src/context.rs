/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Settings;
use fluidity::prelude::State;
use serde::{Deserialize, Serialize};
use tokio::runtime::Handle;
use tracing_subscriber::{fmt, EnvFilter};

#[derive(Clone, Debug)]
pub struct Context {
    cnf: Settings,
    handle: Handle,
    state: State,
}

impl Context {
    pub fn new(cnf: Settings, handle: Handle, state: State) -> Self {
        Self { cnf, handle, state }
    }

    pub fn handle(&self) -> Handle {
        self.handle.clone()
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

impl Default for Context {
    fn default() -> Self {
        let cnf = Settings::default();
        let handle = Handle::current();
        let state = State::default();
        Self { cnf, handle, state }
    }
}

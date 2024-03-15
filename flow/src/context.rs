/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Settings;
use fluidity::prelude::State;
use tokio::runtime::Handle;

pub type PlatformState = State<StateData>;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StateData {
    pub command: String,
}

#[derive(Clone, Debug)]
pub struct Context {
    cnf: Settings,
    handle: Handle,
    state: PlatformState,
}

impl Context {
    pub fn new(cnf: Settings, handle: Handle, state: PlatformState) -> Self {
        Self { cnf, handle, state }
    }

    pub fn handle(&self) -> Handle {
        self.handle.clone()
    }

    pub fn settings(&self) -> Settings {
        self.cnf.clone()
    }

    pub fn state(&self) -> PlatformState {
        self.state.clone()
    }

    pub fn state_mut(&mut self) -> &mut PlatformState {
        &mut self.state
    }

    pub fn set_state(&mut self, state: PlatformState) {
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

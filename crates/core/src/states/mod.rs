/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::state::*;

mod state;

pub trait IntoState {
    type State;

    fn into_state(self) -> Self::State;
}

pub trait Stateful {
    type State;

    fn state(&self) -> &Self::State;
}

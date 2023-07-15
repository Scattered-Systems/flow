/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{power::*, state::*};

mod power;
mod state;

pub trait StateSpec<T> {
    fn message(&self) -> &T;
    fn state(&self) -> &Self {
        self
    }
}

pub trait StateSpace<T> {
    fn states(&self) -> Vec<T>;
}

pub enum BaseState {
    Invalid(),
    Valid(),
}

pub trait StateTransition<T> {
    fn transition(&self, state: T) -> T;
}

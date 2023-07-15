/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{power::*, state::*};

mod power;
mod state;

pub trait StateSpec<T> {
    fn content(&self) -> Option<T>;
    fn state(&self) -> &Self {
        self
    }
}

pub trait StateSpace<T> {
    fn states(&self) -> Vec<T>;
}

pub enum BaseState {
    Invalid(),
    Valid()
}
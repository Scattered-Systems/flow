/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::state::*;

mod state;

pub mod signals;

pub trait IntoState {}

pub trait Stateful {
    type State;

    fn state(&self) -> &Self::State;
}

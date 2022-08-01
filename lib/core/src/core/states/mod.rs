/*
    Appellation: states <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use state::*;

mod state;

pub trait Stateful<S> {
    fn state(&self) -> S
        where
            Self: Sized;
}

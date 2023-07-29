/*
    Appellation: config <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::context::*;

pub(crate) mod context;

pub trait Configuration {}

pub trait Configurable {
    fn configure(self) -> Self;
}

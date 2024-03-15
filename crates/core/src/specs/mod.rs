/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod events;
pub mod execute;
pub mod funcs;
pub mod handler;

pub trait IntoInner {
    type Item;

    fn into_inner(self) -> Self::Item;
}

pub(crate) mod prelude {
    pub use super::events::Eventful;
    pub use super::execute::*;
    pub use super::funcs::*;
    pub use super::handler::*;
    pub use super::IntoInner;
}

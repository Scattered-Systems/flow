/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{events::*, execute::*, funcs::*, handler::*};

pub(crate) mod events;
pub(crate) mod execute;
pub(crate) mod funcs;
pub(crate) mod handler;

pub trait IntoInner {
    type Item;

    fn into_inner(self) -> Self::Item;
}

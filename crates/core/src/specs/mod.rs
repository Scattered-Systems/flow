/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{events::*, execute::*, funcs::*, handler::*};

pub(crate) mod events;
pub(crate) mod execute;
pub(crate) mod funcs;
pub(crate) mod handler;

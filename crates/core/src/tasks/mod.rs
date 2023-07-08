/*
   Appellation: tasks <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{manager::*, registry::*, task::*};

mod manager;
mod registry;
mod task;

pub const DEFAULT_GROUP_NAME: &str = "default";


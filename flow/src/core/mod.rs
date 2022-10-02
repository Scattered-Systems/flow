/*
   Appellation: core <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::{interface::Application, settings::*};

pub mod api;
pub mod cli;
pub mod context;
mod interface;
mod settings;

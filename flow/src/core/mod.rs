/*
   Appellation: core <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::{context::Context, interface::Application, settings::*};

pub mod api;
pub mod cli;

mod context;
mod interface;
mod settings;

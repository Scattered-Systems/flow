/*
   Appellation: app <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::{interface::Application, states::*};

pub mod api;
pub mod cli;
pub mod rpc;

pub(crate) mod interface;
mod states;
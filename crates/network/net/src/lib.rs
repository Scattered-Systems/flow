/*
   Appellation: fluidity-net <library>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Fluidity Net
///
/// This library implements the adaptive peer-to-peer networking stack.
pub use self::{errors::*, primitives::*, specs::*, utils::*};

pub use fluidity_minis as minis;

pub mod conduct;
pub mod config;
pub mod nodes;
pub mod peers;

mod errors;
mod primitives;
mod specs;
mod utils;

pub mod prelude {
    pub use super::conduct::{mainnet::*, Conduct};
    pub use super::config::*;
    pub use super::errors::*;
    pub use super::minis::*;
    pub use super::nodes::*;
    pub use super::peers::*;
    pub use super::primitives::*;
    pub use super::specs::*;
    pub use super::utils::*;
}

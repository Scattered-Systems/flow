/*
   Appellation: fluidity-core <library>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Fluidity Core
///
/// This library is responsible for implementing and maintaining the core features of the Fluidity SDK
pub use self::{primitives::*, utils::*};

mod primitives;
mod utils;

pub mod config;
pub mod errors;
pub mod power;
pub mod specs;
pub mod states;

pub mod prelude {
    pub use super::config::*;
    pub use super::errors::*;
    pub use super::power::*;
    pub use super::primitives::*;
    pub use super::specs::*;
    pub use super::states::*;
    pub use super::utils::*;
}

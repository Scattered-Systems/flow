/*
   Appellation: fluidity-core <library>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Fluidity Core
///
/// This library is responsible for implementing and maintaining the core features of the Fluidity SDK
pub use self::{primitives::*, specs::*, utils::*};

mod primitives;
mod specs;
mod utils;

pub mod errors;
pub mod events;
#[cfg(feature = "frames")]
pub mod frames;
pub mod states;

pub mod prelude {
    pub use super::errors::*;
    pub use super::events::*;
    #[cfg(feature = "frames")]
    pub use super::frames::*;
    pub use super::primitives::*;
    pub use super::specs::*;
    pub use super::states::*;
    pub use super::utils::*;
}

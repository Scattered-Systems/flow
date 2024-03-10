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

pub mod errors;
pub mod events;
pub mod signals;
pub mod specs;
pub mod states;
pub mod stores;

pub mod prelude {
    pub use crate::errors::*;
    pub use crate::events::*;
    pub use crate::primitives::*;
    pub use crate::signals::{power::*, proceed::*};
    pub use crate::specs::*;
    pub use crate::states::*;
    pub use crate::utils::*;
}

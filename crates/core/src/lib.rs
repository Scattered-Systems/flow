/*
   Appellation: fluidity-core <library>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Fluidity Core
//!
//!
#![feature(fn_traits, unboxed_closures)]

pub use self::{primitives::*, utils::*};

mod primitives;
mod utils;

pub mod errors;
pub mod id;
pub mod signals;
pub mod specs;
pub mod states;
pub mod status;
pub mod stores;

pub mod prelude {
    pub use crate::primitives::*;
    pub use crate::utils::*;

    pub use crate::errors::*;
    pub use crate::id::prelude::*;
    pub use crate::signals::prelude::*;
    pub use crate::specs::prelude::*;
    pub use crate::states::*;
}

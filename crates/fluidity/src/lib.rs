/*
   Appellation: fluidity <library>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Fluidity
//!
//!

#[cfg(feature = "core")]
pub use fluidity_core as core;

pub mod orch;
pub mod platform;
pub mod tasks;
pub mod venv;

pub mod prelude {

    #[cfg(feature = "core")]
    pub use crate::core::prelude::*;
}

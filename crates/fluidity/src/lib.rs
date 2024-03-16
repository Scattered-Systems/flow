/*
   Appellation: fluidity <library>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Fluidity
//!
//!
#![feature(fn_traits, trivial_bounds, unboxed_closures)]
#[cfg(feature = "core")]
pub use fluidity_core as core;

pub mod ctrl;
pub mod orch;
pub mod platform;
pub mod tasks;
pub mod venv;

pub mod prelude {
    #[cfg(feature = "core")]
    pub use crate::core::prelude::*;
}

/*
   Appellation: fluidity <library>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Fluidity
///
/// Fluidity implements a novel harmonic orchestration mechanism alongside an adaptive peer-to-peer
/// networking stack.
///
/// ## Features
///
/// - [x] [Fluidity Core](core)
///

#[cfg(feature = "core")]
pub use fluidity_core as core;
#[cfg(feature = "net")]
pub use fluidity_net as net;

pub mod prelude {

    #[cfg(feature = "core")]
    pub use super::core::*;
    #[cfg(feature = "net")]
    pub use super::net::prelude::*;
}

/*
    Appellation: fluidity <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[doc(inline)]
#[cfg(feature = "core")]
pub use fluidity_core as core;
#[cfg(feature = "derive")]
pub use fluidity_derive::*;
#[cfg(feature = "macros")]
pub use fluidity_macros::*;

pub mod prelude {
    #[cfg(feature = "core")]
    pub use super::core::prelude::*;
}

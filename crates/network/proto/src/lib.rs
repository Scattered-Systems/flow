/*
   Appellation: fluidity-proto <library>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Fluidity Proto
///
/// This crate implements the sub-protocols that describe the Flow network.
///
/// ## Sub-Protocols
/// - [x] Request-Response
pub use self::errors::*;

mod errors;

pub mod reqres;

pub mod prelude {
    pub use super::errors::*;

    pub use super::reqres::*;
}

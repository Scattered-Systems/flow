/*
   Appellation: fluidity-minis <library>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Fluidity Minis
///
/// This library implements a set of stateful protocols or minis used to build
/// the adaptive peer-to-peer networking stack.
///
/// ## Sub-Protocols
/// - [x] Request-Response
pub use self::{errors::*, events::*};

mod errors;
mod events;

pub mod reqres;

pub mod prelude {
    pub use super::errors::*;
    pub use super::events::*;
    pub use super::reqres::*;
}

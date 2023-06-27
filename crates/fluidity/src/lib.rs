/*
   Appellation: fluidity <library>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::{core::*, primitives::*, utils::*};

mod core;
mod primitives;
mod utils;

#[cfg(feature = "net")]
pub use fluidity_net as net;

pub mod prelude {
    pub use super::*;

    #[cfg(feature = "net")]
    pub use super::net::prelude::*;
}

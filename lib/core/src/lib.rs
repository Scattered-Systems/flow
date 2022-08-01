/*
    Appellation: fluidity-core <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[doc(inline)]
pub use crate::{actors::*, components::*, core::*, data::*};

pub(crate) mod actors;
pub(crate) mod components;
pub(crate) mod core;
pub(crate) mod data;

pub mod prelude {
    pub use super::actors::{access::*, verify};
    pub use super::components::{assets::*, credentials::*, wallets::*};
    pub use super::core::*;
    pub use super::data::models::*;
}

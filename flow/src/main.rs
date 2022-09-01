/*
    Appellation: flow <binary>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[doc(inline)]
pub use self::{actors::*, components::*, core::*, data::*};

mod actors;
mod components;
mod core;
mod data;

use acme::prelude::APISpec;

#[tokio::main]
async fn main() -> scsys::BoxResult {
    Flow::default().run().await
}

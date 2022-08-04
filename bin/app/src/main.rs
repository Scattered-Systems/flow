/*
    Appellation: api <binary> (rs-sandbox)
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use crate::{context::Context, interface::Flow, settings::Settings};

mod api;
mod context;
mod interface;
mod settings;

use acme::prelude::APISpec;

#[tokio::main]
async fn main() -> Result<(), scsys::BoxError> {
    Flow::default().run().await
}

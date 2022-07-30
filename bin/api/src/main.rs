/*
   Appellation: api <binary>
    Creator: FL03 <jo3mccain@icloud.com>
   Description:
       Powered by ENS, Flow is designed to be your last profile empowering users to seamlessly
       control their entire digital identity.
*/
pub use crate::{app::*, core::*};

mod app;
mod core;

#[tokio::main]
async fn main() -> Result<(), scsys::BoxError> {
    let app = FlowAPI::new();
    app.run().await;
    Ok(())
}

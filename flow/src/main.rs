/*
   Appellation: Flow <binary>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...

*/
pub use self::{actors::*, components::*, core::*, data::*};

mod actors;
mod components;
mod core;
mod data;

#[tokio::main]
async fn main() -> scsys::BoxResult {
    let app = Application::default();
    app.run().await.expect("Application startup failed...");
    Ok(())
}

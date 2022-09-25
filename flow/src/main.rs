/*
   Appellation: maximus <binary>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       Maximus is a complete restful api template written in Rust, powered by Axum and Tokio.

*/
pub use self::{actors::*, components::*, core::*, data::*};

mod actors;
mod components;
mod core;
mod data;

#[tokio::main]
async fn main() -> scsys::BoxResult {
    let app = Application::new(None);
    app.run().await.expect("Application startup failed...");
    Ok(())
}

/*
   Appellation: Flow <binary>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...

*/
pub use self::{actors::*, contexts::*, core::*, data::*};

mod actors;
mod contexts;
mod core;
mod data;

#[tokio::main]
async fn main() -> scsys::BoxResult {
    let mut app = Application::default();
    app.logging(None).run().await.expect("Application startup failed...");

    Ok(())
}

/*
   Appellation: Flow <binary>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...

*/
pub use self::{actors::*, components::*, core::*, data::*};
use scsys::core::BoxResult;

mod actors;
mod components;
mod core;
mod data;

#[tokio::main]
async fn main() -> BoxResult {
    let mut app = Application::default();
    app.logging();
    // proxy::spawn_listener().await.expect("Failed to run the spawner");
    app.api().await?;
    app.rpc().await?;

    Ok(())
}

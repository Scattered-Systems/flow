/*
   Appellation: Flow <binary>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...

*/
pub use self::{application::*, context::*, settings::*, states::*};

pub mod api;
pub(crate) mod application;
pub mod cli;
pub(crate) mod context;
pub(crate) mod settings;
pub(crate) mod states;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> scsys::prelude::BoxResult {
    spawn_application_instance().await?;

    Ok(())
}

pub async fn spawn_application_instance() -> scsys::prelude::BoxResult {
    let application = Application::<String>::default();
    application.run().await?;
    Ok(())
}

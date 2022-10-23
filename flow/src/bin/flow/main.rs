/*
   Appellation: Flow <binary>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...

*/
// pub use self::{context::Context, settings::Settings};
// pub(crate) mod context;
// pub(crate) mod settings;


#[tokio::main(flavor = "multi_thread")]
async fn main() -> scsys::BoxResult {
    let app = fluidity::Application::default();
    app.run().await?;
    fluidity::spawn_application_instance().await?;

    Ok(())
}
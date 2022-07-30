/*
   Appellation: flow <binary>
    Creator: FL03 <jo3mccain@icloud.com>
   Description:
       Powered by ENS, Flow is designed to be your last profile empowering users to seamlessly
       control their entire digital identity.
*/
pub use crate::{app::*, core::*};

mod app;
mod core;

use crate::cli::CLISpec;

#[tokio::main]
async fn main() -> Result<(), scsys::BoxError> {
    let application = Flow::new("development".to_string(), "flow".to_string());
    println!("{}", &application);
    application.run().expect("Application startup failed...");
    Ok(())
}

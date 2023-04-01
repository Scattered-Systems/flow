/*
   Appellation: flow <binary>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::{context::*, settings::*};

mod context;
mod settings;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Ok(())
}
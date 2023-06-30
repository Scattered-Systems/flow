/*
   Appellation: flow <binary>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Flow
///
///
pub use self::{context::*, settings::*, states::*};

mod context;
mod settings;
mod states;

pub mod app;
pub mod cli;
pub mod events;

use anyhow::Result;

#[cfg(not(target_family = "wasm32-wasi"))]
#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args_os();
    let _ = args.next().expect("No args");
    println!("{:?}", args);
    app::starter().run().await?;

    Ok(())
}

#[cfg(target_family = "wasm32-wasi")]
#[tokio_wasi::main]
async fn main() -> Result<()> {
    app::starter().run().await?;

    Ok(())
}

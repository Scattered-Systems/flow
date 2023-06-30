/*
   Appellation: flow <binary>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Flow
///
///
// #[cfg_attr(norm, any(target_family = "unix", target_family = "windows", not(target_family = "wasm")))]
// #[cfg_attr(wasi, feature = "wasi", target_os = "wasi")]
// #[cfg_attr(wasm, target_family = "wasm", feature = "wasm", not(target_os = "wasi"))]

pub use self::{context::*, settings::*, states::*};

mod context;
mod settings;
mod states;

pub mod app;
pub mod cli;
pub mod client;
pub mod events;
pub mod frames;

use anyhow::Result;

#[cfg(not(any(feature = "wasi", feature = "wasm", target_family = "wasm", target_os = "wasi")))]
#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args_os();
    let _ = args.next().expect("No args");
    println!("{:?}", args);
    app::starter().run().await?;

    Ok(())
}

#[cfg(any(feature = "wasi", target_os = "wasi"))]
#[tokio_wasi::main]
async fn main() -> Result<()> {
    app::starter().run().await?;

    Ok(())
}

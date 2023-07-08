/*
   Appellation: flow <binary>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Flow
///
/// Flow describes the core node logic that all network participants must run.
/// With the extensive integrations with Proton, Flow essentially becomes a
/// type of headless operating system capable of synchronizing its activites
/// across devices maximizing the users control and available resources.
///
/// ## Harmonics
///
/// Flow incorperates the Neo-Riemannian theory of music to describe the
/// harmonics of the network(s) as well as the execution process of any transactions.
///
///
/// ## Features
///
/// - [x] Flow Client
/// - [x] Flow Cluster
/// - [x] Flow Network
/// - [x] Flow Node
pub use self::{context::*, settings::*, states::*};

mod context;
mod settings;
mod states;

pub mod app;
pub mod cli;
pub mod client;
pub mod events;
pub mod platform;

use anyhow::Result;

#[cfg(not(any(
    feature = "wasi",
    feature = "wasm",
    target_family = "wasm",
    target_os = "wasi"
)))]
#[tokio::main]
async fn main() -> Result<()> {
    app::starter().run().await?;

    Ok(())
}

#[cfg(any(feature = "wasi", target_os = "wasi"))]
#[tokio_wasi::main]
async fn main() -> Result<()> {
    app::starter().run().await?;

    Ok(())
}

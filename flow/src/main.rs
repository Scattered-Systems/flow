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
pub use self::{context::*, settings::*};

mod context;
mod settings;

pub mod app;
pub mod cli;
pub mod events;
pub mod platform;
pub mod rpc;

use anyhow::Result;

use app::Flow;
use events::FlowEvent;
use platform::{client::FlowClient, PlatformCommand};
use fluidity::prelude::Power;
use tokio::sync::{mpsc, watch};



#[tokio::main]
async fn main() -> Result<()> {
    let (app, client) = starter();
    let cmd = PlatformCommand::connect("".to_string());
    let _ = client.commands.send(cmd).await?;
    let _ = app.spawn().await?;

    Ok(())
}

fn starter() -> (Flow, FlowClient) {
    let buffer: usize = 12;
    let mut args = std::env::args_os();
    let _ = args.next().expect("No args");
    let (_, io_rx) = watch::channel::<std::env::ArgsOs>(args);
    let (commands_tx, commands_rx) = mpsc::channel::<PlatformCommand>(buffer);
    let (events_tx, _erx) = mpsc::channel::<FlowEvent>(buffer);

    let (power_tx, _) = watch::channel::<Power>(Default::default());

    let settings = Settings::new(None);
    let app = Flow::new(commands_rx, events_tx, power_tx, settings).init();
    let client = FlowClient::new(commands_tx);
    return (app, client);
}

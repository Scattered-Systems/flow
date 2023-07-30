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
pub use self::{app::*, context::*, settings::*};

mod app;
mod context;
mod settings;

pub mod cli;
pub mod clients;
pub mod events;
pub mod platform;
pub mod rpc;

use clients::FlowClient;
use events::FlowEvent;
use fluidity::prelude::Power;
use platform::PlatformArgs;

use clap::Parser;
use tokio::sync::{mpsc, watch};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cli::FlowCli::parse();
    let (app, client) = starter();
    let _ = app.spawn();
    match cli.command {
        cli::Options::Platform(args) => {
            let _ = client
                .commands()
                .send(args)
                .await
                .expect("Failed to send command");
        }
        _ => {}
    }
    
    Ok(())
}

fn starter() -> (Flow, FlowClient) {
    let buffer: usize = 12;
    let (commands_tx, commands_rx) = mpsc::channel::<PlatformArgs>(buffer);
    let (events_tx, _erx) = mpsc::channel::<FlowEvent>(buffer);

    let (power_tx, _) = watch::channel::<Power>(Default::default());

    let settings = Settings::build().expect("Failed to build settings");
    println!("{:?}", &settings);
    let app = Flow::new(commands_rx, events_tx, power_tx, settings)
        .with_tracing()
        .init();
    let client = FlowClient::new(commands_tx);
    return (app, client);
}

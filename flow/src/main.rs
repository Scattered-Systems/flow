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
use platform::PlatformCommand;

use tokio::sync::{mpsc, watch};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (app, client) = starter();
    let cmd = PlatformCommand::connect("".to_string());
    let _ = app.spawn();
    let _ = client
        .commands()
        .send(cmd)
        .await
        .expect("Failed to send command");
    // let _ = app.spawn().await.expect("Failed to spawn app");
    // tokio::spawn(async move {
    //     let mut client = client.clone();
    //     let _ = client.send(cmd).await.expect("Failed to send command");
    // }).await;

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

    let settings = Settings::build().expect("Failed to build settings");
    println!("{:?}", &settings);
    let app = Flow::new(commands_rx, events_tx, power_tx, settings)
        .with_tracing()
        .init();
    let client = FlowClient::new(commands_tx);
    return (app, client);
}

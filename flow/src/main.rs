/*
   Appellation: flow <binary>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Flow
//!
//! Flow is a command line tool for managing the Flow platform.
pub use self::{app::*, context::*, settings::*};

mod app;
mod context;
mod settings;

pub mod cli;
pub mod clients;
pub mod events;

use cli::args::platform::PlatformCommand;
use clients::FlowClient;
use events::FlowEvent;
use fluidity::prelude::Power;

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

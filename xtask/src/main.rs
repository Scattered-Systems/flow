/*
    Appellation: xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{primitives::*, utils::*};

mod primitives;
mod utils;

pub mod cli;
pub mod workspace;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    cli::CommandLineInterface::new().handler()?;

    Ok(())
}

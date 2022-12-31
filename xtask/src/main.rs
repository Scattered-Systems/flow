/*
    Appellation: xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#[doc(inline)]
pub use self::{primitives::*, utils::*};

pub(crate) mod primitives;
pub(crate) mod utils;

pub mod cli;
pub mod workspace;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    cli::new().handler()?;

    Ok(())
}

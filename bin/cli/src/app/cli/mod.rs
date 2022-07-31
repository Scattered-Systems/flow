/*
    Appellation: cli <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use args::*;
pub use opts::*;

mod args;
mod opts;

use clap::Parser;

#[derive(Clone, Debug, Hash, Parser, PartialEq, serde::Deserialize, serde::Serialize)]
#[clap(about, author, version)]
pub struct FlowCLI {
    #[clap(arg_enum)]
    pub action: Args,
    #[clap(subcommand)]
    pub option: Opts,
}

impl FlowCLI {
    pub fn run() -> Self {
        Self::parse()
    }
}

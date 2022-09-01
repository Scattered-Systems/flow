/*
    Appellation: cli <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use args::*;
use clap::Parser;
pub use cmds::*;

mod args;
mod cmds;

/// Describes the sandbox command line interface
#[derive(Clone, Debug, Hash, Parser, PartialEq, serde::Deserialize, serde::Serialize)]
#[clap(about, author, version)]
pub struct FlowCLI {
    #[clap(arg_enum)]
    pub action: FlowArgs,
    #[clap(subcommand)]
    pub command: FlowOptions,
}

impl FlowCLI {
    pub fn new() -> Self {
        Self::parse()
    }
    pub fn build() -> Result<Self, scsys::BoxError> {
        let data = Self::default();

        println!("{:#?}", data.clone());
        Ok(data.clone())
    }
}

impl Default for FlowCLI {
    fn default() -> Self {
        Self::new()
    }
}

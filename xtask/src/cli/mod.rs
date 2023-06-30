/*
    Appellation: cli <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{args::*, opts::*};

mod args;
mod opts;

use anyhow::Result;
use clap::Parser;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, Parser, PartialEq, PartialOrd, Serialize)]
#[clap(about, author, long_about = None, version)]
#[command(arg_required_else_help(true), allow_missing_positional(true))]
pub struct CommandLineInterface {
    #[clap(subcommand)]
    pub command: Option<Commands>,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub update: bool,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub verbose: bool,
}

impl CommandLineInterface {
    pub fn new() -> Self {
        Self::parse()
    }
    pub fn handler(&self) -> Result<&Self> {
        if self.verbose {}

        if let Some(cmds) = &self.command {
            cmds.handler()?;
        }
        Ok(self)
    }
}

impl Default for CommandLineInterface {
    fn default() -> Self {
        Self::parse()
    }
}


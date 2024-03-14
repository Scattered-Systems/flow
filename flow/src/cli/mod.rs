/*
   Appellation: cli <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::opts::*;

pub mod args;
mod opts;

use crate::Context;
use clap::Parser;
use serde::{Deserialize, Serialize};

pub fn new() -> Cli {
    Cli::parse()
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, Parser, PartialEq, PartialOrd, Serialize)]
#[clap(about, author, long_about = None, version)]
#[command(arg_required_else_help(true), allow_missing_positional(true))]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Options>,
    #[clap(long, short, default_value_t = String::from("Flow.toml"))]
    pub config: String,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub update: bool,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub verbose: bool,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }

    pub fn command(&self) -> Option<&Options> {
        self.command.as_ref()
    }

    pub fn config(&self) -> &str {
        &self.config
    }

    pub fn update(&self) -> bool {
        self.update
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

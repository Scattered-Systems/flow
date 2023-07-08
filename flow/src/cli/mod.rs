/*
   Appellation: cli <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::opts::*;

pub mod args;
mod opts;

use clap::Parser;
use serde::{Deserialize, Serialize};

pub fn new() -> FlowCli {
    FlowCli::parse()
}



#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, Parser, PartialEq, PartialOrd, Serialize)]
#[clap(about, author, long_about = None, version)]
#[command(arg_required_else_help(true), allow_missing_positional(true))]
pub struct FlowCli {
    #[clap(subcommand)]
    pub command: Options,
    #[clap(long, short, default_value_t = String::from("Flow.toml"))]
    pub config: String,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub update: bool,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub verbose: bool,
}

impl FlowCli {
    
}

/*
    Appellation: interface <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use super::{Commands, Power};
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, Parser, PartialEq, Serialize)]
#[clap(about, author, version)]
#[clap(long_about = "")]
pub struct CommandLineInterface {
    #[clap(subcommand)]
    pub command: Option<Commands>,
    #[arg(long, short)]
    pub update: Option<isize>,
}

impl CommandLineInterface {
    pub fn handler(&self) -> &Self {
        self
    }
}

impl Default for CommandLineInterface {
    fn default() -> Self {
        Self::parse()
    }
}

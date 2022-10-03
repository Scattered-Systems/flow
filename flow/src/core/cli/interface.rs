/*
    Appellation: interface <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use clap::Parser;
use serde::{Deserialize, Serialize};
use super::{Commands, Power};

#[derive(Clone, Debug, Deserialize, Hash, Parser, PartialEq, Serialize)]
#[clap(about, author, version)]
#[clap(long_about = "")]
pub struct CommandLineInterface {
    #[arg(value_enum)]
    pub control: Option<Power>,
    #[clap(subcommand)]
    pub command: Option<Commands>,
    #[arg(long, short)]
    pub update: isize

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

/*
    Appellation: commands <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Subcommand)]
pub enum Commands {
    Account {
        #[clap(long, short, value_parser)]
        address: String,
    },
}

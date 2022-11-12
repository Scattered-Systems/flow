/*
    Appellation: commands <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use super::Power;
use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Subcommand)]
pub enum Commands {
    Account {
        #[clap(long, short, value_parser)]
        address: String,
    },
    Services {
        #[arg(value_enum)]
        power: Option<Power>
    }
}

impl Commands {
    pub fn handler(&self) -> &Self {
        match self {
            Self::Account { address } => {},
            Self::Services { power } => {
                match power.clone() {
                    Some(v) => {
                        match v.clone() {
                            Power::On => {},
                            Power::Off => {}
                        }
                    },
                    None => {}
                }

            }
        };
        self
    }
}

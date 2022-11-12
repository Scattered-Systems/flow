/*
    Appellation: commands <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use super::Power;
use clap::Subcommand;
use scsys::prelude::BoxResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Subcommand)]
pub enum Commands {
    Account {
        #[clap(long, short, value_parser)]
        address: String,
    },
    Services {
        #[arg(value_enum)]
        power: Option<Power>,
    },
}

impl Commands {
    pub async fn handler(&self) -> &Self {
        match self {
            Self::Account { address: _ } => {}
            Self::Services { power } => match power.clone() {
                Some(v) => match v.clone() {
                    Power::On => {
                        crate::spawn_application_instance().await.expect("");
                    }
                    Power::Off => {}
                },
                None => {}
            },
        };
        self
    }
}

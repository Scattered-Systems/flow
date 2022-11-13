use std::sync::Arc;

/*
    Appellation: commands <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use super::Power;
use crate::{api::Api, Application};
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
        #[arg(long, short)]
        update: Option<isize>,
    },
    System {
        #[arg(value_enum)]
        power: Option<Power>,
    },
}

impl Commands {
    pub async fn handler(&self) -> &Self {
        match self {
            Self::Account { address } => {
                println!("{:?}", &address);
            }
            Self::Services { update } => {
                println!("{:?}", &update);

            },
            Self::System { power } => match power.clone() {
                Some(v) => match v.clone() {
                    Power::On => {
                        tracing::info!("Spawning the api...");
                        // tokio::spawn(async move {app.spawn_api();});
                        let api = Api::default();
                        match api.run().await {
                            Err(e) => panic!("{}", e),
                            Ok(v) => v
                        };

                    }
                    Power::Off => {}
                },
                None => {}
            },
        };
        self
    }
}


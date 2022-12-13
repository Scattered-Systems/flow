/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{copy_dir_all, dist_dir, execute_bundle, project_root, Bundle};
use clap::{Args, ArgGroup, Subcommand, ValueEnum};
use duct::cmd;
use scsys::prelude::BoxResult;
use std::process::Command;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, ValueEnum)]
pub enum Setup {
    Desktop,
    Extras,
    #[default]
    Langspace,
}

#[derive(Clone, Debug, Hash, PartialEq, Subcommand)]
pub enum Commands {
    Compile {
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        workspace: bool,
    },
    Create {
        #[clap(long, short, value_parser)]
        name: String,
    },
    Setup {
        #[clap(value_enum)]
        mode: Option<Setup>,
    },
    Start {},
}

impl Commands {
    pub fn handler(&self, desktop: bool, release: bool) -> BoxResult<&Self> {
        tracing::info!("Processing commands issued to the cli...");
        match self {
            Self::Compile { workspace } => {
                tracing::info!("Compiling the codebase...");
            }
            Self::Create { name } => {
                tracing::info!("Creating a new project: {}", name.clone());
            }
            Self::Setup { mode } => {
                tracing::info!("Setting up the environment...");
            }
            Self::Start { } => {
                tracing::info!("Initializing the application server...");
            }
        };
        Ok(self)
    }
}

/*
    Appellation: builder <args>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::command;
use anyhow::Result;
use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Args, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Builder {
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    release: bool,
    #[arg(action = clap::ArgAction::SetFalse, long, short)]
    workspace: bool,
}

impl Builder {
    pub fn new(release: bool, workspace: bool) -> Self {
        Self { release, workspace }
    }
    pub fn handler(&self) -> Result<&Self> {
        tracing::info!("Building the workspace...");
        command("wasm-pack", vec!["build", "flow"])?;
        Ok(self)
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new(false, true)
    }
}
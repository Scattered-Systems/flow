/*
    Appellation: setup <args>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{cargo, command, dist_dir, npm, rustup};
use anyhow::Result;
use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Args, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Setup {
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    extras: bool,
}

impl Setup {
    pub fn new(extras: bool) -> Self {
        Self { extras }
    }
    fn artifacts(&self) -> Result<&Self> {
        if std::fs::create_dir_all(&dist_dir()).is_err() {
            tracing::info!("Clearing out the previous build");
            std::fs::remove_dir_all(&dist_dir())?;
            std::fs::create_dir_all(&dist_dir())?;
        };
        Ok(self)
    }
    fn environment(&self) -> Result<&Self> {
        npm(vec!["install", "-g", "wasm-pack"])?;
        cargo(vec!["install", "wasm-bindgen-cli"])?;
        Ok(self)
    }
    fn langspace(&self) -> Result<&Self> {
        rustup(vec!["default", "nightly"])?;
        rustup(
            vec![
                "target",
                "add",
                "wasm32-unknown-unknown",
                "wasm32-wasi",
                "--toolchain",
                "nightly",
            ],
        )?;
        if self.extras {
            rustup(
                vec![
                    "component",
                    "add",
                    "clippy",
                    "rustfmt",
                    "--toolchain",
                    "nightly",
                ],
            )?;
        };
        Ok(self)
    }
    pub fn handler(&self) -> Result<&Self> {
        tracing::info!("Setting up the workspace...");
        self.artifacts()?.environment()?.langspace()
    }
}

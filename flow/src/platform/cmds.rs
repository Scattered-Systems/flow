/*
    Appellation: platform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(
    Clone, Debug, Default, Deserialize, Eq, Hash, Ord, Parser, PartialEq, PartialOrd, Serialize,
)]
pub struct PlatformArgs {
    #[clap(subcommand)]
    pub command: Option<PlatformCommand>,
}

impl std::fmt::Display for PlatformArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[derive(
    Clone, Debug, Deserialize, Display, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Subcommand
)]
#[strum(serialize_all = "lowercase")]
pub enum PlatformCommand {
    Connect {
        #[clap(long, short)]
        target: Option<String>,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_command() {
        let args = PlatformCommand::Connect { target: Some("10".to_string())};
        assert_eq!(args.to_string(), "connect");
        println!("{}", args);
    }
}
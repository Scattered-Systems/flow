/*
    Appellation: cli <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{commands::*, interface::*};

pub(crate) mod commands;

pub fn new() -> CommandLineInterface {
    CommandLineInterface::default()
}

pub async fn handler(cli: &CommandLineInterface) -> scsys::AsyncResult {
    let cli = cli.clone();

    if let Some(command) = cli.command {
        match command {
            Commands::Account { address } => todo!(),
            Commands::Services { update } => todo!(),
            Commands::System { up } => todo!(),
        }
    };

    Ok(())
}

pub(crate) mod interface {
    use super::Commands;
    use clap::Parser;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Hash, Parser, PartialEq, Serialize)]
    #[clap(about, author, version)]
    #[clap(long_about = "Flow implements in inner-core of all valid nodes operating remote surfaces harmonically with an extensively developed isomorphic state-channel management system")]
    pub struct CommandLineInterface {
        #[clap(subcommand)]
        pub command: Option<Commands>,
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        pub debug: bool,
        #[clap(long, short, value_parser)]
        pub mode: Option<String>,
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        pub update: bool,
    }

    impl CommandLineInterface {
        pub async fn handler(&self) -> scsys::AsyncResult<&Self> {
            match self.command.clone() {
                None => {}
                Some(v) => {
                    v.handler().await?;
                }
            }
            Ok(self)
        }
    }

    impl Default for CommandLineInterface {
        fn default() -> Self {
            Self::parse()
        }
    }
}

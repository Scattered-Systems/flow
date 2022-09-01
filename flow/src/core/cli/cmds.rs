/*
    Appellation: cmds <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::cli::args::WalletAction;

#[derive(clap::Subcommand, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum FlowOptions {
    Wallet {
        #[clap(arg_enum)]
        context: WalletAction,
        #[clap(default_value = "", long, short, value_parser)]
        filepath: String,
        #[clap(default_value = "", long, short, value_parser)]
        label: String,
    },
}

impl Default for FlowOptions {
    fn default() -> Self {
        Self::Wallet {
            context: WalletAction::default(),
            filepath: String::new(),
            label: String::new(),
        }
    }
}

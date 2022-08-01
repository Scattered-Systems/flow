/*
    Appellation: cmds <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::cli::args::WalletArgs;

#[derive(clap::Subcommand, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum FlowOptions {
    Point {
        #[clap(arg_enum)]
        action: WalletArgs,
        #[clap(default_value_t = 0, long, short, value_parser)]
        amount: usize,
    },
}

impl Default for FlowOptions {
    fn default() -> Self {
        Self::Point {
            action: WalletArgs::default(),
            amount: 0,
        }
    }
}

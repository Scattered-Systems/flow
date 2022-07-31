/*
    Appellation: opts <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[derive(clap::ArgEnum, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum WalletArgs {
    Create,
}

#[derive(clap::Subcommand, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Opts {
    Authorize {
        #[clap(default_value = "", long, required = false, short, value_parser)]
        address: String,
        #[clap(default_value = "", long, required = false, short, value_parser)]
        transaction: String,
    },
    Wallet {
        #[clap(arg_enum)]
        action: WalletArgs,
    },
}

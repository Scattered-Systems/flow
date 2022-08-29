/*
    Appellation: args <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[derive(clap::ArgEnum, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum FlowArgs {
    Account,
    Sign,
}

impl Default for FlowArgs {
    fn default() -> Self {
        Self::Account
    }
}

#[derive(
    clap::ArgEnum, Clone, Copy, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub enum WalletAction {
    Create,
    Login,
    Manage,
    Transact,
}

impl Default for WalletAction {
    fn default() -> Self {
        Self::Manage
    }
}

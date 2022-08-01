/*
    Appellation: args <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[derive(clap::ArgEnum, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum FlowArgs {
    Connect,
    Generate,
}

impl FlowArgs {
    pub fn actor(data: Self) -> Self {
        match data {
            Self::Connect => Self::Connect,
            Self::Generate => Self::Generate,
        }
    }
}

impl Default for FlowArgs {
    fn default() -> Self {
        Self::Generate
    }
}

#[derive(
clap::ArgEnum, Clone, Copy, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub enum WalletArgs {
    Access,
    Generate,
}

impl Default for WalletArgs {
    fn default() -> Self {
        Self::Generate
    }
}

/*
    Appellation: interface <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::cli::{FlowArgs, FlowCLI, FlowOptions, WalletAction};
use acme::prelude::CLISpec;
use scsys::BoxError;

#[derive(Clone, Copy, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum AppState {
    Off,
    On,
}

impl Default for AppState {
    fn default() -> Self {
        Self::Off
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Flow {
    pub state: AppState,
    pub timestamp: i64,
}

impl Flow {
    fn build(state: AppState, timestamp: i64) -> Self {
        Self { state, timestamp }
    }
    pub fn new(state: AppState) -> Self {
        Self::build(state, scsys::Utc::now().timestamp())
    }
}

impl Default for Flow {
    fn default() -> Self {
        Self::new(AppState::default())
    }
}

impl CLISpec<FlowCLI> for Flow {
    fn run(&self) -> Result<(), BoxError>
        where
            Self: Sized,
    {
        let data = FlowCLI::default();
        println!("{:#?}", data.clone());
        match data.action {
            FlowArgs::Account => match data.command {
                FlowOptions::Wallet {
                    context,
                    filepath,
                    label,
                } => {
                    if context == WalletAction::Create {
                        let keypair = fluidity::prelude::WalletKey::generate_keypair();
                        let wallet = fluidity::prelude::Wallet::from(&keypair.1, &keypair.0, label);
                        wallet
                            .save_to_file(filepath.as_str())
                            .expect("Interface error");
                        println!("Created a new wallet at: {}", filepath.clone());
                    }
                }
            },
            FlowArgs::Sign => {}
        };

        Ok(())
    }
}

/*
    Appellation: cli <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::cli;
use fluidity::prelude::{Wallet, WalletKey};

pub fn handle_cli_inputs(data: cli::FlowCLI) {
    match data.action {
        cli::FlowArgs::Account => match data.command {
            cli::FlowOptions::Wallet {
                context,
                filepath,
                label,
            } => {
                if context == cli::WalletAction::Create {
                    let keypair = WalletKey::generate_keypair();
                    let wallet = Wallet::from(&keypair.1, &keypair.0, label);
                    wallet
                        .save_to_file(filepath.as_str())
                        .expect("Interface error");
                    println!("Created a new wallet at: {}", filepath.clone());
                }
            }
        },
        cli::FlowArgs::Sign => {}
    };
}

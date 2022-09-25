/*
    Appellation: cli <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

use fluidity::prelude::{Wallet, WalletKey};

pub fn create_wallet() {
    let keypair = WalletKey::generate_keypair();
    let wallet = Wallet::new(&keypair.1, &keypair.0, label);
    wallet
        .save_to_file(filepath.as_str())
        .expect("Interface error");
    println!("Created a new wallet at: {}", filepath.clone());
}
/*
    Appellation: cli <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

use ethers::{signers::{LocalWallet, MnemonicBuilder, Wallet, coins_bip39::English}, prelude::k256::ecdsa::SigningKey};
use scsys::prelude::rand::thread_rng;

pub fn create_wallet() -> LocalWallet {
    LocalWallet::new(&mut thread_rng())
}

pub fn from_passphrase(mnemonic: &str) -> Wallet<SigningKey> {
    match MnemonicBuilder::<English>::default()
        .phrase(mnemonic)
        .build() {
            Ok(v) => v,
            Err(_) => panic!("Failed to setup the wallet...")
        }
}
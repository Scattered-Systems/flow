/*
    Appellation: wallets <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use grant::*;
pub use wallet::*;

mod grant;
mod wallet;

/// Outlines the minimum requirements for creating ERC20 tokens
pub trait ERC20Spec: Clone + std::fmt::Debug {
    fn decimals(&self) -> usize;
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn supply(&self) -> usize;
}

/// Defines a key for use in Crypto Wallets
#[derive(Clone, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
pub struct WalletKey {
    pub public: String,
    pub secret: String,
}

impl WalletKey {
    fn constructor(public: String, secret: String) -> Self {
        Self { public, secret }
    }
    pub fn generate_keypair() -> crate::SecpKeypair {
        let secp = secp256k1::Secp256k1::new();
        secp.generate_keypair(&mut rand::rngs::OsRng)
    }
    // TODO: Find a better method of converting a SecretKey into a String
    pub fn from_keypair(keypair: crate::SecpKeypair) -> Self {
        Self::new(keypair.1.to_string(), format!("{:?}", keypair.0))
    }
    pub fn new(public: String, secret: String) -> Self {
        Self::constructor(public, secret)
    }
}

/// Defines a Web3 Account
pub struct Web3Account {
    pub address: String,
    pub balance: usize,
    pub ensname: String,
}

impl Web3Account {
    fn constructor(address: String, balance: usize, ensname: String) -> Self {
        Self {
            address,
            balance,
            ensname,
        }
    }
    pub fn new(address: String, balance: usize, ensname: String) -> Self {
        Self::constructor(address, balance, ensname)
    }
    pub fn get_balance(&self) -> usize {
        todo!()
    }
}

impl Default for Web3Account {
    fn default() -> Self {
        Self::new(String::new(), 0, String::new())
    }
}

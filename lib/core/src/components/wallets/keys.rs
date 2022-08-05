/*
    Appellation: key <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use rand::rngs::OsRng;
use scsys::{Deserialize, Serialize, Temporal};
use secp256k1::Secp256k1;

/// Defines a key for use in Crypto Wallets
#[derive(Clone, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub struct WalletKey {
    pub public: String,
    pub secret: String,
    pub timestamp: i64,
}

impl WalletKey {
    fn constructor(public: String, secret: String, timestamp: i64) -> Self {
        Self {
            public,
            secret,
            timestamp,
        }
    }
    pub fn generate_keypair() -> crate::SecpKeypair {
        let secp = Secp256k1::new();
        secp.generate_keypair(&mut OsRng)
    }
    // TODO: Find a better method of converting a SecretKey into a String
    pub fn from_keypair(keypair: crate::SecpKeypair) -> Self {
        Self::new(keypair.1.to_string(), format!("{:?}", keypair.0))
    }
    pub fn new(public: String, secret: String) -> Self {
        let timestamp: i64 = Temporal::now().timestamp();
        Self::constructor(public, secret, timestamp)
    }
}

impl Default for WalletKey {
    fn default() -> Self {
        Self::from_keypair(Self::generate_keypair())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_keys() {
        let actual = WalletKey::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}

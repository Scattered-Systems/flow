/*
    Appellation: key <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::SecpKeypair;
use scsys::Timestamp;
use secp256k1::Secp256k1;

/// Defines a key for use in Crypto Wallets
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WalletKey {
    pub public: String,
    pub secret: String,
    pub timestamp: Timestamp,
}

impl WalletKey {
    pub fn new(public: String, secret: String) -> Self {
        Self {
            public,
            secret,
            timestamp: Timestamp::default(),
        }
    }
    pub fn generate_keypair() -> SecpKeypair {
        Secp256k1::new().generate_keypair(&mut rand::rngs::OsRng)
    }
    // TODO: Find a better method of converting a SecretKey into a String
    pub fn from_keypair(keypair: SecpKeypair) -> Self {
        Self::new(keypair.1.to_string(), format!("{:?}", keypair.0))
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

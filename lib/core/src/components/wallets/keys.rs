/*
    Appellation: key <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use scsys::Timestamp;
use secp256k1::Secp256k1;

/// Defines a key for use in Crypto Wallets
#[derive(Clone, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
pub struct WalletKey {
    pub public: String,
    pub secret: String,
    pub timestamp: Timestamp,
}

impl WalletKey {
    fn constructor(public: String, secret: String, timestamp: Timestamp) -> Self {
        Self {
            public,
            secret,
            timestamp,
        }
    }
    pub fn generate_keypair() -> crate::SecpKeypair {
        Secp256k1::new().generate_keypair(&mut rand::rngs::OsRng)
    }
    // TODO: Find a better method of converting a SecretKey into a String
    pub fn from_keypair(keypair: crate::SecpKeypair) -> Self {
        Self::new(keypair.1.to_string(), format!("{:?}", keypair.0))
    }
    pub fn new(public: String, secret: String) -> Self {
        Self::constructor(public, secret, Timestamp::new())
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

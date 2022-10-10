/*
    Appellation: interface <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::wallets::keys::WalletKey;
use secp256k1::{PublicKey, SecretKey};
use std::str::FromStr;

/// Describes the standard wallets interface for digital currencies
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Wallet {
    pub address: String,
    pub key: WalletKey,
    pub label: String,
}

impl Wallet {
    pub fn new(label: String) -> Self {
        let address: String = String::new();
        let key = WalletKey::default();

        Self {
            address,
            key,
            label,
        }
    }
    pub fn from_file(file_path: &str) -> scsys::BoxResult<Self> {
        let file = std::fs::OpenOptions::new().read(true).open(file_path)?;
        let buf_reader = std::io::BufReader::new(file);
        let wallet: Wallet = serde_json::from_reader(buf_reader)?;
        Ok(wallet)
    }
    pub fn public_key(&self) -> scsys::BoxResult<PublicKey> {
        Ok(PublicKey::from_str(&self.key.public)?)
    }
    pub fn save_to_file(&self, path: &str) -> scsys::BoxResult<Self> {
        crate::save_to_file(self.clone(), path)
    }
    pub fn secret_key(&self) -> scsys::BoxResult<SecretKey> {
        Ok(SecretKey::from_str(&self.key.secret)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet() {
        let actual = Wallet::new("test".to_string());
        let expected = actual.clone();
        assert_eq!(
            &format!("{:#?}", actual.key),
            &format!("{:#?}", expected.key)
        );
        assert_eq!(
            &format!("{:#?}", actual.label),
            &format!("{:#?}", expected.label)
        )
    }
}

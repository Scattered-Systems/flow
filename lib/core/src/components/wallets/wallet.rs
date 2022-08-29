/*
    Appellation: interface <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{public_key_address, wallets::keys::WalletKey, Web3Address};
use secp256k1::{PublicKey, SecretKey};
use std::str::FromStr;

/// Describes the standard wallets interface for digital currencies
#[derive(Clone, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
pub struct Wallet {
    pub address: String,
    pub key: WalletKey,
    pub label: String,
}

impl Wallet {
    fn constructor(address: String, key: WalletKey, label: String) -> Self {
        Self {
            address,
            label,
            key,
        }
    }
    pub fn new(label: String) -> Self {
        let key = WalletKey::default();
        let address: Web3Address = public_key_address(
            &PublicKey::from_str(key.public.clone().as_str())
                .ok()
                .unwrap(),
        );
        Self::constructor(format!("{}", address), key, label)
    }
    pub fn from(public: &PublicKey, secret: &SecretKey, label: String) -> Self {
        Self::constructor(
            public_key_address(&public).to_string(),
            WalletKey::new(public.to_string(), format!("{:?}", secret)),
            label,
        )
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

/*
    Appellation: interface <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use secp256k1::{PublicKey, SecretKey};
use std::str::FromStr;

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
    pub fn from_keypair(keypair: crate::SecpKeypair) -> Self {
        Self::new(keypair.1.to_string(), format!("{:?}", keypair.0))
    }
    pub fn new(public: String, secret: String) -> Self {
        Self::constructor(public, secret)
    }
}

impl Default for WalletKey {
    fn default() -> Self {
        todo!()
    }
}

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
        let keypair = WalletKey::generate_keypair();
        let address: web3::types::Address = crate::public_key_address(&keypair.1);
        Self::constructor(
            format!("{:?}", address),
            WalletKey::from_keypair(keypair),
            label,
        )
    }
    pub fn from(public: &PublicKey, secret: &SecretKey, label: String) -> Self {
        let addr: web3::types::Address = crate::public_key_address(&public);
        Self::constructor(
            format!("{:?}", addr),
            WalletKey::new(public.to_string(), format!("{:?}", secret)),
            label,
        )
    }
    pub fn save_to_file(&self, file_path: &str) -> anyhow::Result<()> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;
        let buf_writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(buf_writer, self)?;
        Ok(())
    }
    pub fn from_file(file_path: &str) -> anyhow::Result<Self> {
        let file = std::fs::OpenOptions::new().read(true).open(file_path)?;
        let buf_reader = std::io::BufReader::new(file);
        let wallet: Wallet = serde_json::from_reader(buf_reader)?;
        Ok(wallet)
    }
    pub fn get_secret_key(&self) -> anyhow::Result<SecretKey> {
        Ok(SecretKey::from_str(&self.key.secret)?)
    }
    pub fn get_public_key(&self) -> anyhow::Result<PublicKey> {
        Ok(PublicKey::from_str(&self.key.public)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet() {
        let actual = Wallet::new("test".to_string());
        let expected = actual.clone();
        println!("{:#?}", actual.clone());
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

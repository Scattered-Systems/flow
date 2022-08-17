/*
    Appellation: mnemonic <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{generate_collection_from_reference, mnemonics::Passphrase};

#[derive(Clone, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
pub struct Mnemonic {
    pub mnemonic: Vec<String>,
    pub passphrase: String,
}

impl Mnemonic {
    fn constructor(mnemonic: Vec<String>, passphrase: String) -> Self {
        Self { mnemonic, passphrase }
    }
    pub fn new(passphrase: String) -> Self {
        let mnemonic = generate_collection_from_reference(crate::BIP0039::default().data, 12);
        Self::constructor(mnemonic, passphrase)
    }
    pub fn passphrase(&self) -> String {
        self.passphrase.clone()
    }
    pub fn salt(&self) -> String {
        let salt = String::new();
        self.passphrase() + salt.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mnemonic() {
        let actual = Mnemonic::new(Passphrase::default().0);
        let expected = Mnemonic::new(Passphrase::default().0);
        assert_ne!(actual, expected)
    }
}
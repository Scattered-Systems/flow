/*
    Appellation: mnemonic <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use scsys::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Mnemonic {
    pub passphrase: String,
}

impl Mnemonic {
    fn constructor(passphrase: String) -> Self {
        Self { passphrase }
    }
    pub fn new(passphrase: String) -> Self {
        Self::constructor(passphrase)
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
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        assert_eq!(f(4, 2), 6)
    }
}

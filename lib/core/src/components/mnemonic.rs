/*
    Appellation: mnemonic <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[derive(Clone, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
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

#[derive(Clone, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
pub struct Passphrase(String);

impl Passphrase {
    fn constructor(data: String) -> Self {
        Self(data)
    }
    pub fn new(data: String) -> Self {
        Self::constructor(data)
    }
    pub fn from<T: std::string::ToString>(data: T) -> Self {
        Self::new(data.to_string())
    }
    pub fn generate(len: usize) -> Self {
        Self::new(crate::generate_random_string(len))
    }
}

impl Default for Passphrase {
    fn default() -> Self {
        Self::generate(12)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passphrase() {
        let actual = Passphrase::default();
        let expected = Passphrase::default();
        assert_ne!(actual, expected)
    }
}

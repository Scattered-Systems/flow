pub use self::mnemonic::*;

mod mnemonic;

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

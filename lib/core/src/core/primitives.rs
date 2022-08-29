/*
    Appellation: primitives <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{constants::*, types::*};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Language {
    English,
    French,
}

impl Language {
    pub fn new(data: String) -> Self {
        Self::related()
            .get(data.clone().to_lowercase().as_str())
            .clone()
            .expect("")
            .clone()
    }
    pub fn from(data: Self) -> String {
        match data {
            Language::English => Self::english(),
            Language::French => Self::french(),
        }
    }
    pub fn related() -> scsys::Dictionary<Self> {
        let data = [
            ("english".to_string(), Self::English),
            ("french".to_string(), Self::French),
        ];
        scsys::Dictionary::from(data.clone())
    }

    pub fn english() -> String {
        "english".to_string()
    }
    pub fn french() -> String {
        "french".to_string()
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct BIP0039(pub Vec<String>);

impl BIP0039 {
    pub fn new(data: Vec<String>) -> Self {
        Self(data)
    }
    pub async fn fetch(lang: Language) -> scsys::BoxResult<Self> {
        let response = reqwest::get(format!(
            "{}/{}.txt",
            BIP0039_WORDLIST_ENDPOINT,
            Language::from(lang)
        ))
        .await?
        .text()
        .await?;
        let mut data = response.split("\n").collect::<Vec<_>>();
        data.retain(|&x| x != "");
        let res = data.iter().map(|i| i.to_string()).collect();
        Ok(Self::new(res))
    }
    pub fn from_file(path: &str) -> Self {
        let mut data = crate::extract_file_from_path(path);
        data.retain(|x| x != &"".to_string());
        Self::new(data)
    }
}

impl Default for BIP0039 {
    fn default() -> Self {
        Self::from_file(PATH_TO_BIP0039_DATA)
    }
}

mod constants {
    /// Define the valid sizes of generated access grants
    pub const ACCESS_GRANT_VALID_BIT_SIZES: [usize; 5] = [128, 160, 192, 224, 256];
    /// Define the default filepath for locating the BIP0039 english text file
    pub const PATH_TO_BIP0039_DATA: &str = "../../.artifacts/data/BIP0039/english.txt";
    /// Define the endpoint pointing to BIP0039 Mnemonics
    pub const BIP0039_WORDLIST_ENDPOINT: &str =
        "https://raw.githubusercontent.com/bitcoin/bips/master/bip-0039";
}

mod types {
    use secp256k1::{PublicKey, SecretKey};

    /// Type alias for a tuple ([secp256k1::SecretKey], [secp256k1::PublicKey])
    pub type SecpKeypair = (SecretKey, PublicKey);
    /// Type alias for a Web3 address [web3::Address]
    pub type Web3Address = web3::types::Address;
    /// Type alias for [web3::Web3] leveraging an HTTP transport [web3::transports::Http]
    pub type Web3Http = web3::Web3<web3::transports::Http>;
    /// Type alias for [anyhow::Result] of type [Web3Http]
    pub type Web3HttpResult = anyhow::Result<Web3Http>;
    /// Type alias for [web3::Web3] leveraging an WebSocket transport [web3::transports::WebSocket]
    pub type Web3WS = web3::Web3<web3::transports::WebSocket>;
    /// Type alias for [anyhow::Result] of type [Web3WS]
    pub type Web3WSResult = anyhow::Result<Web3WS>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_wordlist_english() {
        let actual = BIP0039::fetch(Language::English).await.ok().unwrap();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}

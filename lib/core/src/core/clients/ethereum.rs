/*
    Appellation: ethereum <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
pub struct Web3Client {
    pub endpoint: String,
    pub version: String,
}

impl Web3Client {
    fn constructor(endpoint: String, version: String) -> Self {
        Self { endpoint, version }
    }
    pub async fn connect(&self) -> anyhow::Result<web3::Web3<web3::transports::Http>> {
        connect_to_web3(self.endpoint.clone().as_str()).await
    }
    pub fn new(endpoint: String, version: String) -> Self {
        Self::constructor(endpoint, version)
    }
}

impl Default for Web3Client {
    fn default() -> Self {
        Self::new("https://rpc.ankr.com/eth".to_string(), String::new())
    }
}

/// Quickly connect to Web3 Providers
pub async fn connect_to_web3(endpoint: &str) -> anyhow::Result<web3::Web3<web3::transports::Http>> {
    Ok(web3::Web3::new(
        web3::transports::Http::new(endpoint).ok().unwrap(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web3_client_default() {
        let actual = Web3Client::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}

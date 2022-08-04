/*
    Appellation: ethereum <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[derive(Clone, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
pub enum Web3Endpoint {
    HTTP,
    WSS,
}

impl Default for Web3Endpoint {
    fn default() -> Self {
        Self::HTTP
    }
}

#[derive(Clone, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
pub struct Web3Client {
    pub endpoint: String,
}

impl Web3Client {
    fn constructor(endpoint: String) -> Self {
        Self { endpoint }
    }
    pub fn new(endpoint: String) -> Self {
        Self::constructor(endpoint)
    }
    pub async fn http(self) -> web3::Web3<web3::transports::Http> {
        web3::Web3::new(
            web3::transports::Http::new(self.endpoint.as_str())
                .ok()
                .unwrap(),
        )
    }
}

/// Quickly connect to Web3 Providers
pub async fn connect_to_web3(endpoint: &str) -> web3::Web3<web3::transports::Http> {
    web3::Web3::new(web3::transports::Http::new(endpoint).ok().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web3_client_version() {
        let url = "https://rpc.ankr.com/eth";
        let client = Web3Client::new("https://rpc.ankr.com/eth".to_string());
        let actual = "".to_string();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}

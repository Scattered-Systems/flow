/*
    Appellation: provider <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

pub type HttpProvider = ethers::providers::Provider<ethers::providers::Http>;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Provider {
    pub name: String,
    pub endpoint: String,
}

impl Provider {
    pub fn new(name: String, endpoint: String) -> Self {
        Self { name, endpoint }
    }
    pub fn from_settings(settings: &crate::Settings) -> Self {
        Self::new(
            settings.provider.name.clone(),
            settings.provider.endpoint.clone(),
        )
    }
    pub fn http_provider(&self) -> HttpProvider {
        HttpProvider::try_from(self.endpoint.clone().as_str())
            .expect("Failed to connect to the provider...")
    }
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Successfully initialized the Web3 Provider {}",
            self.name
        )
    }
}

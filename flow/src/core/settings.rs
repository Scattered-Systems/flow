/*
   Appellation: settings
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use scsys::{prelude::{config::{Config, Environment}, Cache, Database, Logger, Server, Web3Provider}, collect_config_files};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AppSettings {
    pub mode: Option<String>,
    pub name: Option<String>,
}

impl AppSettings {
    pub fn slug(&self) -> String {
        self.name.clone().unwrap_or_default().to_lowercase()
    }
}

impl std::fmt::Display for AppSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Application",)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]

pub enum Provider {
    Cache(Cache),
    Database(Database),
    Ethereum(Web3Provider)
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Settings {
    pub application: AppSettings,
    pub logger: Option<Logger>,
    pub server: Server,
}

impl Settings {
    pub fn build() -> scsys::ConfigResult<Self> {
        let builder = Config::builder()
            .add_source(collect_config_files("**/Flow.toml", true))
            .add_source(Environment::default().separator("__"));

        builder.build()?.try_deserialize()
    }
}

impl Default for Settings {
    fn default() -> Self {
        match Self::build() {
            Ok(v) => v,
            Err(e) => panic!("Configuration Error: {}", e),
        }
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Configured {}", self.application)
    }
}

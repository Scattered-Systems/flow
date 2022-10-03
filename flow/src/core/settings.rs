/*
   Appellation: settings
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use scsys::{
    collect_config_files,
    prelude::{
        config::{Config, ConfigError, Environment},
        Cache, Database, Logger, Server, Web3Provider,
    },
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Application {
    pub mode: String,
    pub name: String,
}

impl Application {
    pub fn slug(&self) -> String {
        self.name.clone().to_lowercase()
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Application",)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]

pub struct Providers {
    pub cache: Option<Cache>,
    pub database: Option<Database>,
    pub ethereum: Option<Web3Provider>
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Settings {
    pub application: Application,
    pub logger: Option<Logger>,
    pub providers: Providers,
    pub server: Server,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut builder = Config::builder();

        builder = builder.add_source(collect_config_files("**/flow.config.*", true));
        builder = builder.add_source(collect_config_files("**/*.config.*", false));
        builder = builder.add_source(Environment::default().separator("__"));

        builder
            .build()
            .expect("Failed to build the configuration...")
            .try_deserialize()
    }
}

impl Default for Settings {
    fn default() -> Self {
        match Self::new() {
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

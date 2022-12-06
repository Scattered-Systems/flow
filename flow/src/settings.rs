/*
   Appellation: settings
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use scsys::prelude::config::{Config, Environment};
use scsys::Hashable;
use scsys::{prelude::*, try_collect_config_files, ConfigResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Hashable, PartialEq, Serialize)]
pub struct Settings {
    pub mode: Option<String>,
    pub name: Option<String>,
    pub logger: Logger,
    pub server: Server,
}

impl Settings {
    pub fn new(mode: Option<String>) -> Self {
        let name = Some("Flow".to_string());
        let logger = Logger::default();
        let server = Server::new("127.0.0.1".to_string(), 9090);
        Self {
            mode,
            name,
            logger,
            server,
        }
    }
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Config::builder()
            .add_source(Environment::default().separator("__"))
            .set_default("logger.level", "info")?
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 9090)?;
        match try_collect_config_files("**/Puzzled.toml", false) {
            Err(_) => {}
            Ok(v) => {
                builder = builder.add_source(v);
            }
        }
        match std::env::var("CLIENT_ID") {
            Err(_) => {}
            Ok(v) => {
                builder = builder.set_override("client_id", Some(v))?;
            }
        };
        let (pk, sk) = (std::env::var("CLIENT_ID"), std::env::var("CLIENT_SECRET"));
        if pk.is_ok() {
            builder = builder.set_override("client_secret", Some(pk.ok().unwrap()))?;
        }
        if sk.is_ok() {
            builder = builder.set_override("client_secret", Some(sk.ok().unwrap()))?;
        };
        let lvl = std::env::var("RUST_LOG");
        if lvl.is_ok() {
            builder = builder.set_override("logger.level", lvl.ok().unwrap())?;
        }
        let port = std::env::var("RUST_LOG");
        if port.is_ok() {
            builder = builder.set_override("server.port", port.ok().unwrap())?;
        }

        match std::env::var("SERVER_PORT") {
            Err(_) => {}
            Ok(v) => {
                builder = builder.set_override("server.port", v)?;
            }
        };

        builder.build()?.try_deserialize()
    }
}

impl Configurable for Settings {
    type Settings = Self;

    fn settings(&self) -> &Self::Settings {
        self
    }
}

impl Default for Settings {
    fn default() -> Self {
        match Self::build() {
            Ok(v) => v,
            Err(_) => Self::new(Some("dev".to_string())),
        }
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

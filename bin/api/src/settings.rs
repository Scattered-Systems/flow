/*
    Appellation: settings <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use acme::prelude::{Database, HostPiece, Logger, PortPiece, SocketAddrPieces};

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Settings {
    pub application: AppSettings,
    pub database: Database,
    pub logger: Logger,
    pub server: ServerParams,
}

impl Settings {
    pub fn constructor() -> Result<scsys::DefaultConfigBuilder, config::ConfigError> {
        let mut builder = config::Config::builder()
            .set_default("application.mode", "development")?
            .set_default("application.name", "rs-sandbox")?
            .set_default("database.name", "postgres")?
            .set_default(
                "database.uri",
                "postgres://postgres:example@localhost:5432/postgres",
            )?
            .set_default("logger.level", "info")?
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?;

        builder = builder.add_source(scsys::collect_config_files("**/*.config.*", false));
        builder = builder.add_source(config::Environment::default().separator("__"));
        Ok(builder)
    }
    pub fn new() -> Result<Self, config::ConfigError> {
        Self::constructor().ok().unwrap().build()?.try_deserialize()
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AppSettings {
    pub mode: String,
    pub name: String,
}

impl std::fmt::Display for AppSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Application(mode={}, name={})", self.mode, self.name)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ServerParams {
    pub host: String,
    pub port: PortPiece,
}

impl ServerParams {
    fn constructor(host: String, port: PortPiece) -> Self {
        Self { host, port }
    }
    pub fn address(self) -> std::net::SocketAddr {
        std::net::SocketAddr::from(self.pieces())
    }
    pub fn new(host: String, port: PortPiece) -> Self {
        Self::constructor(host, port)
    }
    pub fn pieces(self) -> SocketAddrPieces {
        let host: HostPiece = scsys::extract::Extractor::new(',', self.host.clone())
            .extract()
            .try_into()
            .ok()
            .unwrap();
        (host, self.port)
    }
}

impl Default for ServerParams {
    fn default() -> Self {
        Self::new("0.0.0.0".to_string(), 8080)
    }
}

impl std::fmt::Display for ServerParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "View the server locally at http://localhost:{}",
            self.port
        )
    }
}
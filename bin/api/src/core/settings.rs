/*
    Appellation: settings <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use components::*;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct APISettings {
    pub application: Application,
    pub database: Database,
    pub logger: Logger,
    pub server: Server,
}

impl APISettings {
    pub fn constructor(
    ) -> Result<config::ConfigBuilder<config::builder::DefaultState>, config::ConfigError> {
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

mod components {
    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Application {
        pub mode: String,
        pub name: String,
    }

    impl std::fmt::Display for Application {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Application(mode={}, name={})", self.mode, self.name)
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Database {
        pub name: String,
        pub uri: String,
    }

    impl std::fmt::Display for Database {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Database(name={}, uri={})", self.name, self.uri)
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Logger {
        pub level: String,
    }

    impl Logger {
        fn constructor(level: String) -> Self {
            Self { level }
        }
        pub fn new(level: String) -> Self {
            Self::constructor(level)
        }
        pub fn setup(configuration: &crate::APISettings) {
            if std::env::var_os("RUST_LOG").is_none() {
                let level = configuration.logger.level.as_str();
                std::env::set_var("RUST_LOG", level);
            }

            tracing_subscriber::fmt::init();
        }
    }

    impl Default for Logger {
        fn default() -> Self {
            Self::new("info".to_string())
        }
    }

    impl std::fmt::Display for Logger {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Logger(level={})", self.level)
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Server {
        pub host: String,
        pub port: u16,
    }

    impl Server {
        fn constructor(host: String, port: u16) -> Self {
            Self { host, port }
        }
        pub fn address(self) -> std::net::SocketAddr {
            std::net::SocketAddr::from(self.pieces())
        }
        pub fn new(host: String, port: u16) -> Self {
            Self::constructor(host, port)
        }
        pub fn pieces(self) -> ([u8; 4], u16) {
            let host: [u8; 4] = scsys::extract::Extractor::new(',', self.host.clone())
                .extract()
                .try_into()
                .ok()
                .unwrap();
            (host, self.port)
        }
    }

    impl std::fmt::Display for Server {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Server(host={}, port={})", self.host, self.port)
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Application {
    pub name: String,
    pub slug: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Database {
    pub name: String,
    pub uri: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Provider {
    pub endpoint: String,
    pub public: String,
    pub secret: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Server {
    pub host: [u8; 4],
    pub port: u16,
}

impl std::fmt::Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "View the application locally at http://localhost:{}", self.port)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Settings {
    pub application: Application,
    pub database: Database,
    pub logger: Logger,
    pub provider: Provider,
    pub server: Server,
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "--- Settings ---\n\t{}", self.server)
    }
}
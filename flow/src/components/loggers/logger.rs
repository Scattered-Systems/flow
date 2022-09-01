/*
    Appellation: logger <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Logger {
    pub level: String,
}

impl Logger {
    pub fn setup(settings: &crate::Settings) {
        if std::env::var_os("RUST_LOG").is_none() {
            let level = settings.logger.level.as_str();
            std::env::set_var("RUST_LOG", level);
        }

        tracing_subscriber::fmt::init();
    }
}

impl std::fmt::Display for Logger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Application logging set to {}", self.level)
    }
}

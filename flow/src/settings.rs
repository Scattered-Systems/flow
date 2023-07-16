/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use config::{Config, Environment};
use decanter::prelude::Hashable;
use scsys::prelude::{try_collect_config_files, ConfigResult, SerdeDisplay};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumIter,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Hashable,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[repr(u8)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    #[default]
    Development = 0,
    Staging = 1,
    Production = 2,
}

#[derive(
    Clone,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Hashable,
    Ord,
    PartialEq,
    PartialOrd,
    SerdeDisplay,
    Serialize,
)]
pub struct Logger {
    pub level: String,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            level: tracing::Level::INFO.to_string(),
        }
    }
    pub fn set_level(mut self, level: impl ToString) {
        self.level = level.to_string();
    }
    pub fn setup_env(mut self) -> Self {
        let key = "RUST_LOG";
        if let Some(v) = std::env::var_os(key) {
            self.level = v.into_string().expect("Failed to convert into string...");
        } else {
            std::env::set_var(key, self.level.clone());
        }
        self
    }
    pub fn init_tracing(self) {
        self.setup_env();
        tracing_subscriber::fmt::init();
        tracing::debug!("Success: tracing layer initialized...");
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

impl From<tracing::Level> for Logger {
    fn from(level: tracing::Level) -> Self {
        Self {
            level: level.to_string(),
        }
    }
}

#[derive(
    Clone,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Hashable,
    Ord,
    PartialEq,
    PartialOrd,
    SerdeDisplay,
    Serialize,
)]
pub struct Settings {
    pub logger: Logger,
    pub mode: Mode,
}

impl Settings {
    pub fn new(logger: Option<Logger>, mode: Option<Mode>) -> Self {
        Self {
            logger: logger.unwrap_or_default(),
            mode: mode.unwrap_or_default(),
        }
    }
    pub fn builder() -> config::ConfigBuilder<config::builder::DefaultState> {
        Config::builder()
    }
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Self::builder()
            .set_default("mode", "development")?
            .set_default("logger.level", "info")?;

        if let Ok(log) = std::env::var("RUST_LOG") {
            builder = builder.set_override("logger.level", log)?;
        };
        // Add in related environment variables
        builder = builder.add_source(
            Environment::default()
                .separator("__")
                .prefix(env!("CARGO_PKG_NAME").to_ascii_uppercase().as_str()),
        );
        // Try gathering valid configuration files...
        if let Ok(files) = try_collect_config_files("**/Flow.toml", false) {
            builder = builder.add_source(files);
        }
        builder.build()?.try_deserialize()
    }

    pub fn logger(&self) -> Logger {
        self.logger.clone()
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }
}

impl Default for Settings {
    fn default() -> Self {
        let d = Self::new(None, Mode::Development.into());
        Self::build().unwrap_or(d)
    }
}

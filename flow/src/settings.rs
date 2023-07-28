/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use config::{Config, Environment, File};
use decanter::prelude::Hashable;
use scsys::prelude::{try_collect_config_files, ConfigResult, SerdeDisplay};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
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
pub enum LogLevel {
    #[default]
    Debug = 0,
    Error = 1,
    Info = 2,
    Trace = 3,
    Warn = 4,
}

impl From<tracing::Level> for LogLevel {
    fn from(level: tracing::Level) -> Self {
        use LogLevel::*;
        match level {
            tracing::Level::DEBUG => Debug,
            tracing::Level::ERROR => Error,
            tracing::Level::INFO => Info,
            tracing::Level::TRACE => Trace,
            tracing::Level::WARN => Warn,
        }
    }
}

impl From<LogLevel> for tracing::Level {
    fn from(level: LogLevel) -> tracing::Level {
        use LogLevel::*;
        match level {
            Debug => tracing::Level::DEBUG,
            Error => tracing::Level::ERROR,
            Info => tracing::Level::INFO,
            Trace => tracing::Level::TRACE,
            Warn => tracing::Level::WARN,
        }
    }
}

#[derive(
    Clone,
    Copy,
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
    level: LogLevel,
}

impl Logger {
    pub fn new(level: LogLevel) -> Self {
        Self {
            level,
        }
    }
    pub fn from_env() -> Self {
        if let Ok(v) = std::env::var("RUST_LOG") {
            let level = LogLevel::from_str(&v).unwrap_or_default();
            return Self::new(level);
        }
        Self::default()
    }
    pub fn level(&self) -> LogLevel {
        self.level
    }
    pub fn set_level(mut self, level: LogLevel) {
        self.level = level;
    }
    pub fn setup_env(mut self) -> Self {
        std::env::set_var("RUST_LOG", self.level.to_string());
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
        Self::new(LogLevel::default())
    }
}

impl From<tracing::Level> for Logger {
    fn from(level: tracing::Level) -> Self {
        Self {
            level: level.into(),
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
        builder = builder.add_source(File::with_name("Flow").required(false));
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

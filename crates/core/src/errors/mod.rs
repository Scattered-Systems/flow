/*
   Appellation: errors <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumIter, EnumVariantNames};

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumIter,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
)]
pub enum FlowError {
    Connection(String),
    #[default]
    Error(String),
    Platform(String),
    Runtime(String),
}

impl std::error::Error for FlowError {}

impl From<&str> for FlowError {
    fn from(err: &str) -> Self {
        Self::Error(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow_error() {
        let err = FlowError::Error("test".to_string());
        assert_eq!(err, "test".into());
    }
}

/*
   Appellation: errors <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::error::*;

pub(crate) mod error;

pub mod kinds;

use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumIter, VariantNames};

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
    VariantNames,
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
    use super::kinds::ErrorKind;

    #[test]
    fn test_error_kind() {
        let err = ErrorKind::custom("test".to_string());
        assert_eq!(err, "test".into());
    }
}

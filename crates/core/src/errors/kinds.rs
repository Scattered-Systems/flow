/*
   Appellation: kinds <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
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
pub enum ErrorKind {
    #[default]
    Error(String),
    Unknown,
}

impl std::error::Error for ErrorKind {}

impl From<&str> for ErrorKind {
    fn from(err: &str) -> Self {
        Self::Error(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_kind() {
        let err = ErrorKind::Error("test".to_string());
        assert_eq!(err, "test".into());
    }
}

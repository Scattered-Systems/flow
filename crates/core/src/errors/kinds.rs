/*
   Appellation: kinds <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Error Kinds
//!
//!
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, EnumIter, VariantNames};

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize,),
    serde(rename_all = "snake_case", untagged)
)]
#[derive(
    Clone,
    Debug,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    SmartDefault,
    VariantNames,
)]
#[strum(serialize_all = "snake_case")]
pub enum ErrorKind {
    #[default]
    Error(ExternalError),
}

impl ErrorKind {
    pub fn custom(err: impl ToString) -> Self {
        Self::Error(ExternalError::custom(err))
    }

    pub fn unknown() -> Self {
        Self::Error(ExternalError::unknown())
    }
}

impl std::error::Error for ErrorKind {}

macro_rules! impl_from_error {
    ($ty:ty) => {
        impl From<$ty> for ErrorKind {
            fn from(err: $ty) -> Self {
                Self::Error(err.into())
            }
        }
    };
    ($($ty:ty),*) => {
        $(
            impl_from_error!($ty);
        )*
    };
}

impl_from_error!(String, &str);
impl_from_error!(ExternalError);

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumCount,
    EnumIs,
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
#[serde(rename_all = "snake_case", untagged)]
#[strum(serialize_all = "snake_case")]
pub enum ExternalError {
    #[default]
    Error(String),
    Unknown,
}

impl ExternalError {
    pub fn custom(err: impl ToString) -> Self {
        Self::Error(err.to_string())
    }

    pub fn unknown() -> Self {
        Self::Unknown
    }
}

impl std::error::Error for ExternalError {}

impl From<&str> for ExternalError {
    fn from(err: &str) -> Self {
        Self::Error(err.to_string())
    }
}

impl From<String> for ExternalError {
    fn from(err: String) -> Self {
        Self::Error(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_kind() {
        let err = ErrorKind::custom("test".to_string());
        assert_eq!(err, "test".into());
    }
}

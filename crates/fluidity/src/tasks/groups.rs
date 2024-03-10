/*
   Appellation: groups <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::DEFAULT_GROUP_NAME;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, EnumIter, VariantNames};

#[derive(
    Clone,
    Copy,
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
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum GroupName {
    Custom(&'static str),
    #[default]
    Default,
}

impl GroupName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => DEFAULT_GROUP_NAME,
            Self::Custom(name) => name,
        }
    }
}

impl AsRef<str> for GroupName {
    fn as_ref(&self) -> &str {
        match self {
            Self::Default => DEFAULT_GROUP_NAME,
            Self::Custom(name) => name,
        }
    }
}

impl From<&'static str> for GroupName {
    fn from(name: &'static str) -> Self {
        match name {
            DEFAULT_GROUP_NAME => Self::Default,
            _ => Self::Custom(name),
        }
    }
}
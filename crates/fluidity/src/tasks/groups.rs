/*
   Appellation: groups <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::DEFAULT_GROUP_NAME;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, EnumIter, VariantNames};

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize,),
    serde(rename_all = "lowercase", untagged)
)]
#[derive(
    Clone,
    Copy,
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
#[strum(serialize_all = "lowercase")]
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

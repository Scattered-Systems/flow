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
    Custom(String),
    #[default]
    Default,
}

impl GroupName {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Default => DEFAULT_GROUP_NAME,
            Self::Custom(name) => name.as_str(),
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

impl From<&str> for GroupName {
    fn from(name: &str) -> Self {
        if name == DEFAULT_GROUP_NAME {
            Self::Default
        } else {
            Self::Custom(name.to_string())
        }
    }
}

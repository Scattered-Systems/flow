/*
   Appellation: tasks <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{manager::*, registry::*, task::*};

mod manager;
mod registry;
mod task;

pub const DEFAULT_GROUP_NAME: &str = "default";

use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumIter, EnumString, EnumVariantNames};

#[derive(
   Clone,
   Copy,
   Debug,
   Deserialize,
   Display,
   EnumIter,
   EnumString,
   EnumVariantNames,
   Eq,
   Hash,
   Ord,
   PartialEq,
   PartialOrd,
   Serialize,
   SmartDefault,
)]
#[serde(rename_all = "snake_case")]
pub enum GroupName {
   #[default]
   Default,
   Custom(&'static str),
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

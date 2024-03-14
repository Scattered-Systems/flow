/*
    Appellation: proceed <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Proceed
//!
//!
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize,),
    serde(rename_all = "lowercase", untagged)
)]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    EnumString,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    VariantNames,
)]
#[strum(serialize_all = "lowercase")]
pub enum Proceed {
    #[default]
    Proceed,
    Stop,
}

impl Proceed {
    pub fn proceed() -> Self {
        Proceed::Proceed
    }

    pub fn stop() -> Self {
        Proceed::Stop
    }
}

impl From<Proceed> for usize {
    fn from(proceed: Proceed) -> Self {
        proceed as usize
    }
}

impl From<usize> for Proceed {
    fn from(proceed: usize) -> Self {
        match proceed % Self::COUNT {
            0 => Proceed::Proceed,
            _ => Proceed::Stop,
        }
    }
}

impl From<Proceed> for bool {
    fn from(proceed: Proceed) -> Self {
        match proceed {
            Proceed::Proceed => true,
            Proceed::Stop => false,
        }
    }
}

impl From<bool> for Proceed {
    fn from(proceed: bool) -> Self {
        if proceed {
            Proceed::Proceed
        } else {
            Proceed::Stop
        }
    }
}

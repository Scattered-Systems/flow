/*
    Appellation: proceed <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
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
    Serialize,
    VariantNames,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
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

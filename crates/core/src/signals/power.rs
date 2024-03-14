/*
    Appellation: power <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Power
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
#[repr(u8)]
pub enum Power {
    Off = 0,
    #[default]
    On = 1,
}

impl Power {
    pub fn off() -> Self {
        Self::Off
    }

    pub fn on() -> Self {
        Self::On
    }
}

impl From<Power> for u8 {
    fn from(p: Power) -> Self {
        p as u8
    }
}

impl From<usize> for Power {
    fn from(u: usize) -> Self {
        match u % Self::COUNT {
            1 => Self::On,
            _ => Self::Off,
        }
    }
}

impl From<u8> for Power {
    fn from(u: u8) -> Self {
        Power::from(u as usize)
    }
}

impl From<bool> for Power {
    fn from(b: bool) -> Self {
        match b {
            true => Self::On,
            false => Self::Off,
        }
    }
}

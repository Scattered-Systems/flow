/*
    Appellation: power <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Power
///
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
#[repr(u8)]
pub enum Power {
    Off = 0,
    #[default]
    On = 1,
    
}

impl Power {
    pub fn is_on(&self) -> bool {
        match *self {
            Self::On => true,
            _ => false,
        }
    }
}

impl From<Power> for u8 {
    fn from(p: Power) -> Self {
        match p {
            Power::On => 1,
            Power::Off => 0,
        }
    }
}

impl From<u8> for Power {
    fn from(u: u8) -> Self {
        match u % 2 {
            1 => Self::On,
            _ => Self::Off,
        }
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

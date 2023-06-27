
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumIter, EnumString, EnumVariantNames};

#[derive(Clone, Copy, Debug, Deserialize, Display, EnumIter, EnumString, EnumVariantNames, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, SmartDefault)]
#[repr(u8)]
pub enum PowerEvent {
    #[default]
    On = 0,
    Off = 1,
}

impl PowerEvent {
    pub fn is_on(&self) -> bool {
        match self {
            Self::On => true,
            Self::Off => false,
        }
    }
    pub fn is_off(&self) -> bool {
        match self {
            Self::On => false,
            Self::Off => true,
        }
    }
}

impl From<bool> for PowerEvent {
    fn from(b: bool) -> Self {
        match b {
            true => Self::On,
            false => Self::Off,
        }
    }
}
/*
    Appellation: power <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use clap::ValueEnum;
use scsys::{
    prelude::H256,
    Hashable,
    BoxResult,
};
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Clone, Debug, Deserialize, Hash, Hashable, PartialEq, Serialize)]
pub enum Power {
    On,
    Off,
}

impl Power {
    pub fn handler(&self, catalyst: fn(Self) -> BoxResult) -> BoxResult {
        match self {
            Self::On => catalyst(self.clone()),
            Self::Off => catalyst(self.clone()),
        }
    }
}

impl Default for Power {
    fn default() -> Self {
        Self::Off
    }
}

impl std::fmt::Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

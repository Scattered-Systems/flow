/*
    Appellation: power <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum Power {
    On,
    Off,
}

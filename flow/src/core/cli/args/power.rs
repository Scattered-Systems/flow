/*
    Appellation: power <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use clap::ArgEnum;
use serde::{Deserialize, Serialize};

#[derive(ArgEnum, Clone, Debug, Deserialize, Hash, PartialEq,  Serialize)]
pub enum Power {
    On,
    Off
}


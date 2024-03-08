/*
    Appellation: opts <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::args::platform::PlatformCommand;
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(
    Clone,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
    Subcommand,
)]
pub enum Options {
    #[default]
    Platform(PlatformCommand),
}

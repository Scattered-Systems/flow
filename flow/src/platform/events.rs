/*
    Appellation: platform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Platform Events
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
#[strum(serialize_all = "snake_case")]
pub enum PlatformEvent {
    Connect,
    Initializing,
    #[default]
    Idle,

    Terminate,
}

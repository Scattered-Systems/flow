/*
    Appellation: events <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::platform::*;

mod platform;

use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumIter, EnumString, EnumVariantNames};

#[derive(
    Clone,
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
pub enum FlowEvent {
    Client,
    #[default]
    Platform(PlatformEvent),
}

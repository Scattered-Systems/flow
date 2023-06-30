/*
    Appellation: events <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::power::*;

mod power;

use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumIter, EnumString, EnumVariantNames};

pub trait EventSpec<T> {
    fn event(&self) -> &T;
}

pub trait EventHandler: Send + Sync + 'static {
    fn handle_event(&self, event: FlowEvent) -> anyhow::Result<()>;
}

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
    #[default]
    Power(PowerEvent),
}

impl EventSpec<FlowEvent> for FlowEvent {
    fn event(&self) -> &FlowEvent {
        self
    }
}

impl EventSpec<PowerEvent> for FlowEvent {
    fn event(&self) -> &PowerEvent {
        match self {
            Self::Power(event) => event,
        }
    }
}

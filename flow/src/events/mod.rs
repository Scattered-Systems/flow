
pub use self::power::*;

mod power;

use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumIter, EnumString, EnumVariantNames};



pub trait EventHandle {
    fn handle_event(&self, event: FlowEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}


#[derive(Clone, Copy, Debug, Deserialize, Display, EnumIter, EnumString, EnumVariantNames, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, SmartDefault)]
pub enum FlowEvent {
    #[default]
    Power(PowerEvent),
}

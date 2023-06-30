/*
   Appellation: states <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
/// #
///
///
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
#[repr(u8)]
#[strum(serialize_all = "snake_case")]
pub enum State {
    #[default]
    Idle = 0,
    Processing {
        transaction: String,
    } = 1,
}

impl State {
    /// [State::Idle] variant
    pub fn idle() -> Self {
        Self::Idle
    }
    pub fn processing(transaction: String) -> Self {
        Self::Processing { transaction }
    }
    pub fn is_idle(&self) -> bool {
        match self {
            Self::Idle => true,
            _ => false,
        }
    }
    pub fn is_processing(&self) -> bool {
        match self {
            Self::Processing { .. } => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_idle() {
        let state = State::idle();
        assert!(state.is_idle());
        assert!(!state.is_processing());
        assert_eq!(state.to_string(), "idle")
    }
}

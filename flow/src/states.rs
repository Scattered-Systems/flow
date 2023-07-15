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
    Error(String) = 0,
    #[default]
    Idle = 1,
    Process {
        transaction: String,
    } = 2,
}

impl State {
    /// [State::Error] variant
    pub fn error(message: impl ToString) -> Self {
        Self::Error(message.to_string())
    }
    /// [State::Idle] variant
    pub fn idle() -> Self {
        Self::Idle
    }
    pub fn processing(transaction: String) -> Self {
        Self::Process { transaction }
    }
    pub fn is_idle(&self) -> bool {
        match self {
            Self::Idle => true,
            _ => false,
        }
    }
    pub fn is_processing(&self) -> bool {
        match self {
            Self::Process { .. } => true,
            _ => false,
        }
    }
    pub fn update(&mut self, state: Self) {
        *self = state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_idle() {
        let mut state = State::idle();
        assert!(state.is_idle());
        state.update(State::processing("test".to_string()));
        assert!(state.is_processing())
    }
}

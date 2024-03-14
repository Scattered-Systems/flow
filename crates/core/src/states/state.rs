/*
   Appellation: state <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # State
//!
//!
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct State {
    message: String,
    state: States,
}

impl State {
    pub fn new(message: impl ToString, state: States) -> Self {
        Self {
            message: message.to_string(),
            state,
        }
    }
    /// Sets the state to [States::Invalid]
    pub fn invalidate(&mut self) {
        self.state = States::Invalid;
    }
    /// Returns true if the state is [States::Valid]
    pub fn is_valid(&self) -> bool {
        self.state == States::Valid
    }
    /// Returns the message
    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn set_message(&mut self, message: impl ToString) {
        self.message = message.to_string();
    }
    pub fn set_state(&mut self, state: States) {
        self.state = state;
    }
    /// Returns the current state
    pub fn state(&self) -> States {
        self.state
    }

    pub fn update(&mut self, state: State) {
        *self = state;
    }
}

#[cfg(feature = "serde")]
impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[cfg(not(feature = "serde"))]
impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.state.to_string())
    }
}

impl From<States> for State {
    fn from(state: States) -> Self {
        Self::new("", state)
    }
}

impl From<State> for States {
    fn from(q: State) -> Self {
        q.state
    }
}

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize,),
    serde(rename_all = "lowercase", untagged)
)]
#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    EnumString,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    SmartDefault,
    VariantNames,
)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub enum States {
    Invalid = 0,
    #[default]
    Valid = 1,
}

impl States {
    /// [State::Invalid] variant constructor
    pub fn invalid() -> Self {
        Self::Invalid
    }
    /// [State::Valid] variant constructor
    pub fn valid() -> Self {
        Self::Valid
    }
    ///
    pub fn update(&mut self, state: Self) {
        *self = state;
    }
}

impl From<u8> for States {
    fn from(u: u8) -> Self {
        match u % 2 {
            1 => Self::Valid,
            _ => Self::Invalid,
        }
    }
}

impl From<States> for u8 {
    fn from(s: States) -> Self {
        s as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let mut state = State::new("", States::valid());
        let message: &str = "test";
        assert!(state.is_valid(), "The provided state isn't valid");
        // Set the message
        state.set_message(message);
        // Set the state to invalid
        state.invalidate();
        assert_eq!(
            state.message(),
            message,
            "State Error: the message isn't equal to the assigned message"
        );
        assert!(!state.is_valid())
    }
}

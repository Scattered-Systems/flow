/*
   Appellation: state <module>
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
    Invalid {
        message: String,
    } = 0,
    #[default]
    Valid {
        message: String,
    } = 1,
}

impl State {
    /// [State::Invalid] variant constructor
    pub fn invalid(message: impl ToString) -> Self {
        Self::Invalid { message: message.to_string() }
    }
    /// [State::Valid] variant constructor
    pub fn valid(message: impl ToString) -> Self {
        Self::Valid { message: message.to_string() }
    }
    pub fn is_valid(&self) -> bool {
        match *self {
            Self::Invalid { .. } => false,
            _ => true,
        }
    }
    pub fn message(&self) -> &str {
        match *self {
            Self::Invalid { ref message } => message,
            Self::Valid { ref message } => message,
        }
    }
    pub fn set(&mut self, message: impl ToString) {
        let state = self.with(message);
        *self = state;
    }
    pub fn with(&self, message: impl ToString) -> Self {
        match *self {
            Self::Invalid { .. } => Self::invalid(message),
            _ => Self::valid(message),
        }
    }
    pub fn update(&mut self, state: Self) {
        *self = state;
    }
}

impl AsMut<str> for State {
    fn as_mut(&mut self) -> &mut str {
        match *self {
            Self::Invalid { ref mut message } => message,
            Self::Valid { ref mut message } => message,
        }
    }
}

impl AsRef<str> for State {
    fn as_ref(&self) -> &str {
        self.message()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let mut state = State::valid("");
        assert!(state.is_valid());
        state.set("test");
        assert_eq!(state.message(), "test");
        state.update(State::invalid("test"));
        assert!(!state.is_valid())
    }
}

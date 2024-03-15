/*
   Appellation: state <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # State
//!
//!
use super::BinaryState;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
pub struct State<T = String> {
    data: T,
    state: BinaryState,
}

impl<T> State<T> {
    pub fn new(data: T, state: BinaryState) -> Self {
        Self { data, state }
    }
    /// Sets the state to [States::Invalid]
    pub fn invalidate(&mut self) {
        self.state = BinaryState::Invalid;
    }
    /// Returns true if the state is [States::Valid]
    pub fn is_valid(&self) -> bool {
        self.state == BinaryState::Valid
    }
    /// Returns the message
    pub fn data(&self) -> &T {
        &self.data
    }
    /// Returns the current state
    pub fn state(&self) -> BinaryState {
        self.state
    }
    /// Sets the message
    pub fn set_data(&mut self, message: T) {
        self.data = message;
    }
    /// Sets the state
    pub fn set_state(&mut self, state: BinaryState) {
        self.state = state;
    }

    pub fn with_data(self, data: T) -> Self {
        Self { data, ..self }
    }

    pub fn with_state(self, state: BinaryState) -> Self {
        Self { state, ..self }
    }
}

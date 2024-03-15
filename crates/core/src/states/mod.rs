/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # States
//!
//!
pub use self::{binary::*, state::State};

pub(crate) mod binary;
mod state;

pub trait IntoState {
    type State;

    fn into_state(self) -> Self::State;
}

pub trait Stateful<T> {
    type State;

    fn data(&self) -> &T;

    fn state(&self) -> &Self::State;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let data = "test";
        let mut state = State::<String>::new("".to_string(), BinaryState::valid());
        // Check if the state is valid
        assert!(state.is_valid(), "The provided state isn't valid");
        // Set the message
        state.set_data(data.to_string());
        // Check if the message has been updated
        assert_eq!(
            state.data(),
            data,
            "State Error: the message isn't equal to the assigned message"
        );
        // Set the state to invalid
        state.invalidate();
        // Check if the state has been invalidated
        assert!(!state.is_valid())
    }
}

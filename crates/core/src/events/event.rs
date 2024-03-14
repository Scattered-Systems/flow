/*
   Appellation: event <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Event
//!
//!
use crate::id::atomic::AtomicId;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Event {
    id: AtomicId,
    message: String,
    timestamp: u64,
}

impl Event {
    pub fn new(message: String) -> Self {
        let id = AtomicId::new();
        let timestamp = crate::utils::systime();
        Self {
            id,
            message,
            timestamp,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
        self.on_update();
    }

    fn on_update(&mut self) {
        self.timestamp = crate::utils::systime();
    }
}

unsafe impl Send for Event {}

unsafe impl Sync for Event {}

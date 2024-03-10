/*
   Appellation: event <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Event {
    message: String,
    timestamp: u64,
}

impl Event {
    pub fn new(message: String) -> Self {
        let timestamp = crate::utils::systime();
        Self { message, timestamp }
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

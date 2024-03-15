/*
   Appellation: status <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::StatusCode;
use crate::prelude::{systime, Ts};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
pub struct Status {
    pub(crate) message: String,
    pub(crate) status: StatusCode,
    pub(crate) timestamp: Ts,
}

impl Status {
    pub fn new(message: impl ToString, status: StatusCode) -> Self {
        Self {
            message: message.to_string(),
            status,
            timestamp: systime(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn timestamp(&self) -> Ts {
        self.timestamp
    }

    pub fn set_message(&mut self, message: impl ToString) {
        self.message = message.to_string();
        self.on_update();
    }

    pub fn with_message(mut self, message: impl ToString) -> Self {
        self.message = message.to_string();
        self.on_update();
        self
    }

    pub(crate) fn on_update(&mut self) {
        self.timestamp = systime();
    }
}

#[cfg(feature = "serde")]
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

#[cfg(not(feature = "serde"))]
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "StatusMessage\nMessage: {}\nStatus: {}\nTimestamp: {}",
            self.message, self.status, self.timestamp
        )
    }
}

impl From<StatusCode> for Status {
    fn from(status: StatusCode) -> Self {
        Self {
            message: String::new(),
            status,
            timestamp: systime(),
        }
    }
}

/*
   Appellation: error <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::kinds::{ErrorKind, ExternalError};
use crate::prelude::{systime, Ts};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Error {
    kind: ErrorKind,
    message: String,
    timestamp: Ts,
}

impl Error {
    pub fn new(kind: ErrorKind, message: String) -> Self {
        Self {
            kind,
            message,
            timestamp: systime(),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn timestamp(&self) -> Ts {
        self.timestamp
    }

    pub fn set_kind(&mut self, kind: ErrorKind) {
        self.kind = kind;
        self.on_update();
    }

    fn on_update(&mut self) {
        self.timestamp = systime();
    }
}

unsafe impl Send for Error {}

unsafe impl Sync for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Error\nKind: {}\nTimestamp: {}\n{}",
            self.kind, self.timestamp, self.message
        )
    }
}

impl std::error::Error for Error {}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self::new(ErrorKind::unknown(), message)
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Self::new(ErrorKind::unknown(), message.to_string())
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self::new(kind.clone(), kind.to_string())
    }
}

macro_rules! impl_error_from {
    ($from:ty, $kind:expr) => {
        impl From<$from> for Error {
            fn from(err: $from) -> Self {
                Self::new(ErrorKind::from($kind), err.to_string())
            }
        }
    };
}

type BoxError = Box<dyn std::error::Error>;
impl_error_from!(BoxError, ExternalError::Unknown);

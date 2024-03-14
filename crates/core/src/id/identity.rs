/*
    Appellation: identity <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::atomic::AtomicId;
use crate::prelude::systime;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
pub struct Id<T = AtomicId> {
    id: T,
    timestamp: u64,
}

impl<T> Id<T> {
    pub fn new(id: T) -> Self {
        Self {
            id,
            timestamp: systime(),
        }
    }

    pub fn id(&self) -> &T {
        &self.id
    }

    pub fn set(&mut self, id: T) {
        self.id = id;
        self.on_update();
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn on_update(&mut self) {
        self.timestamp = systime();
    }
}

impl<T> AsRef<T> for Id<T> {
    fn as_ref(&self) -> &T {
        &self.id
    }
}

impl<T> Deref for Id<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

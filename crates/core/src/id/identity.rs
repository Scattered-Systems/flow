/*
    Appellation: identity <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::atomic::AtomicId;
use super::Identifier;
use crate::prelude::{systime, Ts};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
pub struct Id<T = AtomicId> {
    id: T,
    timestamp: Ts,
}

impl<T> Id<T> {
    pub fn new(id: T) -> Self {
        Self {
            id,
            timestamp: systime(),
        }
    }

    pub fn into_inner(self) -> T {
        self.id
    }

    pub fn get(&self) -> &T {
        &self.id
    }

    pub fn set(&mut self, id: T) {
        self.id = id;
        self.on_update();
    }

    pub fn timestamp(&self) -> Ts {
        self.timestamp
    }

    fn on_update(&mut self) {
        self.timestamp = systime();
    }
}

impl<T> Identifier for Id<T> {
    type Id = T;
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

impl<T> Default for Id<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> std::fmt::Display for Id<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl<T> From<T> for Id<T> {
    fn from(id: T) -> Self {
        Self::new(id)
    }
}

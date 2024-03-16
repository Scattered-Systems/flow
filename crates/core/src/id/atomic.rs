/*
    Appellation: atomic <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Atomic Id
//!
//!
use crate::sync::atomic::AtomicOrder;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicUsize;

pub fn atomic_id(order: Option<AtomicOrder>) -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, order.unwrap_or_default().into())
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
pub struct AtomicId {
    order: AtomicOrder,
    value: usize,
}

impl AtomicId {
    pub fn new(order: AtomicOrder) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        let id = COUNTER.fetch_add(1, order.into());
        Self { order, value: id }
    }

    pub fn relaxed() -> Self {
        Self::new(AtomicOrder::Relaxed)
    }

    pub fn into_inner(self) -> usize {
        self.value
    }

    pub fn get(&self) -> usize {
        self.value
    }

    pub fn next(&self) -> Self {
        Self::new(self.order)
    }

    pub fn set(&mut self, id: usize) {
        self.value = id;
    }
}

impl AsRef<usize> for AtomicId {
    fn as_ref(&self) -> &usize {
        &self.value
    }
}

impl AsMut<usize> for AtomicId {
    fn as_mut(&mut self) -> &mut usize {
        &mut self.value
    }
}

impl Default for AtomicId {
    fn default() -> Self {
        Self::new(AtomicOrder::Relaxed)
    }
}

impl Deref for AtomicId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for AtomicId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl std::fmt::Display for AtomicId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<AtomicOrder> for AtomicId {
    fn from(order: AtomicOrder) -> Self {
        Self::new(order)
    }
}

impl From<usize> for AtomicId {
    fn from(id: usize) -> Self {
        Self {
            order: AtomicOrder::Relaxed,
            value: id,
        }
    }
}

impl From<AtomicId> for usize {
    fn from(id: AtomicId) -> Self {
        id.value
    }
}

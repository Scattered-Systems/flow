/*
   Appellation: utils <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Utils
//!
//!
use std::time::SystemTime;

pub trait TimeStamp {
    fn now() -> Self;
}

impl TimeStamp for u64 {
    fn now() -> Self {
        SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

impl TimeStamp for u128 {
    fn now() -> Self {
        SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }
}

pub struct Timestamp<T = u128>(T);

impl<T: TimeStamp> Timestamp<T> {
    pub fn new(t: T) -> Self {
        Timestamp(t)
    }

    pub fn now() -> Self {
        Self(T::now())
    }
}

impl<T: TimeStamp> From<T> for Timestamp<T> {
    fn from(t: T) -> Self {
        Timestamp(t)
    }
}

/// [systime] is a utilitarian function that returns the current system time in seconds.
pub fn systime() -> u128 {
    SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

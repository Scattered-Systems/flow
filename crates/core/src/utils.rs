/*
   Appellation: utils <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Utils
//!
//!

/// [systime] is a utilitarian function that returns the current system time in seconds.
pub fn systime() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

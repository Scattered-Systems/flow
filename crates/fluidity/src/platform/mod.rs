/*
   Appellation: platform <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Platform
//!
pub use self::platform::*;

pub(crate) mod platform;

pub mod clients;
pub mod config;
pub mod context;
pub mod rt;

#[cfg(test)]
mod tests {}

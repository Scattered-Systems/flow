/*
    Appellation: ctrl <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Controller
//!
//! Controllers are generally responsible for managing the state of a system.
//!
//! In robotics and automation, a control loop is a non-terminating loop that manages the state of a system.
//! Here, controllers are control loops that manage the state of the unified digital environment.

pub use self::controller::Controller;

pub(crate) mod controller;

#[cfg(test)]
mod tests {}

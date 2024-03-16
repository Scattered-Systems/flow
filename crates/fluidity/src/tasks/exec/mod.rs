/*
   Appellation: exec <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Exec
//!
//! The `exec` module provides the `Executor` type, which is responsible for executing tasks.
pub use self::{executor::*, waker::TaskWaker};

pub(crate) mod executor;
pub(crate) mod waker;

pub trait Worker {
    type Id;

    fn spawn(&self, id: Self::Id);
}

#[cfg(test)]
mod tests {}

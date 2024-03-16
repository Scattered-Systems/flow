/*
   Appellation: tasks <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Tasks
//!
//! The [tasks](crate::tasks) module implements primitives and utilities for managing tasks.

pub use self::{groups::*, id::*, task::*};

pub(crate) mod groups;
pub(crate) mod id;
pub(crate) mod task;

pub mod exec;
pub mod manage;
pub mod stores;
pub mod workloads;

pub const DEFAULT_GROUP_NAME: &str = "default";

#[cfg(test)]
mod tests {
    use super::Task;

    #[test]
    fn test_task() {
        let task = Task::new("test".into(), "test");
        assert!(!task.is_default());
    }
}

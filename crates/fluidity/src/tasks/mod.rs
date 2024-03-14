/*
   Appellation: tasks <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{groups::*, task::*};

pub(crate) mod groups;
pub(crate) mod task;

pub mod manage;
pub mod stores;
pub mod workloads;

pub const DEFAULT_GROUP_NAME: &str = "default";

#[cfg(test)]
mod tests {
    use super::stores::TaskRegistry;
    use super::Task;

    #[test]
    fn test_task_registry() {
        let mut registry = TaskRegistry::new();
        let task = Task::new("test".into(), "test");
        registry.register(task.clone());
        assert_eq!(registry.snapshot().get(&task), Some(&1));
    }
}

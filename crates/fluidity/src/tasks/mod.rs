/*
   Appellation: tasks <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{groups::*, manager::*, registry::*, task::*};

pub(crate) mod groups;
pub(crate) mod manager;
pub(crate) mod registry;
pub(crate) mod task;

pub const DEFAULT_GROUP_NAME: &str = "default";


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_task_registry() {
        let mut registry = TaskRegistry::new();
        let task = Task::new("test".into(), "test");
        registry.register(task.clone());
        assert_eq!(registry.snapshot().get(&task), Some(&1));
    }

    #[tokio::test]
    async fn test_task_manager() {
        let mut manager = TaskManager::new();
        let mut child = TaskManager::new();
        child.push(TaskManager::new());
        manager.push(child);
        assert_eq!(manager.children().len(), 1);
    }
}
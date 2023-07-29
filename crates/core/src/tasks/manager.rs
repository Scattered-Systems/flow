/*
   Appellation: manager <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{IntoRegistry, TaskRegistry};
use tokio::runtime::Handle;

#[derive(Clone, Debug)]
pub struct TaskManager {
    children: Vec<TaskManager>,
    handle: Handle,
    registry: TaskRegistry,
}

impl TaskManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn children(&self) -> Vec<TaskManager> {
        self.children.clone()
    }

    pub fn handle(&self) -> Handle {
        self.handle.clone()
    }

    pub fn push(&mut self, task: TaskManager) {
        self.children.push(task)
    }

    pub fn registry(&self) -> TaskRegistry {
        self.registry.clone()
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            handle: Handle::current(),
            registry: TaskRegistry::new(),
        }
    }
}

impl IntoRegistry for TaskManager {
    fn into_registry(self) -> TaskRegistry {
        self.registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_task_manager() {
        let mut manager = TaskManager::new();
        let mut child = TaskManager::new();
        child.push(TaskManager::new());
        manager.push(child);
        assert_eq!(manager.children().len(), 1);
    }
}

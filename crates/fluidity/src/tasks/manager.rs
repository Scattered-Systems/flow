/*
   Appellation: manager <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::stores::{IntoRegistry, TaskRegistry};
use tokio::runtime::Handle;

#[derive(Clone, Debug)]
pub struct TaskManager {
    children: Vec<TaskManager>,
    handle: Handle,
    registry: TaskRegistry,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            handle: Handle::current(),
            registry: TaskRegistry::new(),
        }
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
        Self::new()
    }
}

impl IntoRegistry for TaskManager {
    fn into_registry(self) -> TaskRegistry {
        self.registry
    }
}

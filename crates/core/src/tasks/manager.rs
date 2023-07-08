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
    pub fn push(&mut self, task: TaskManager) {
        self.children.push(task)
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

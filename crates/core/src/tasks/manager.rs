/*
   Appellation: manager <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::TaskRegistry;
use tokio::runtime::Handle;

pub struct TaskManager {

    children: Vec<TaskManager>,
    handle: Handle,
    registry: TaskRegistry,
}

impl TaskManager {
    pub fn into_registry(self) -> TaskRegistry {
        self.registry
    }

    pub fn push(&mut self, task: TaskManager) {
        self.children.push(task)
    }
}
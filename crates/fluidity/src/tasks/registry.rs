/*
   Appellation: registry <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Task;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub trait IntoRegistry {
    fn into_registry(self) -> TaskRegistry;
}

impl<T> IntoRegistry for T
where
    T: Into<TaskRegistry>,
{
    fn into_registry(self) -> TaskRegistry {
        self.into()
    }
}

pub(crate) type RegistryStore<K = Task, V = usize> = Arc<Mutex<HashMap<K, V>>>;

#[derive(Clone, Debug)]
pub struct TaskRegistry {
    tasks: RegistryStore,
}

impl TaskRegistry {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn register(&mut self, task: Task) {
        let mut tasks = self.tasks();
        let count = tasks.entry(task.clone()).or_insert(0);
        *count += 1;
        self.tasks.lock().unwrap().insert(task, *count);
    }
    pub fn len(&self) -> usize {
        self.tasks().len()
    }
    pub fn tasks(&self) -> HashMap<Task, usize> {
        self.tasks.lock().unwrap().clone()
    }
}

impl Default for TaskRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Arc<Mutex<HashMap<Task, usize>>>> for TaskRegistry {
    fn from(tasks: Arc<Mutex<HashMap<Task, usize>>>) -> Self {
        Self { tasks }
    }
}

impl From<HashMap<Task, usize>> for TaskRegistry {
    fn from(tasks: HashMap<Task, usize>) -> Self {
        Self {
            tasks: Arc::new(Mutex::new(tasks)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_task_registry() {
        let mut registry = TaskRegistry::new();
        let task = Task::new("test", "test");
        registry.register(task.clone());
        let tasks = registry.tasks();
        assert_eq!(tasks.get(&task), Some(&1));
    }
}

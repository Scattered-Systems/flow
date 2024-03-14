/*
   Appellation: registry <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::tasks::Task;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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
    /// Returns the number of tasks in the registry
    pub fn len(&self) -> usize {
        self.tasks.lock().unwrap().len()
    }

    pub fn snapshot(&self) -> HashMap<Task, usize> {
        self.tasks.lock().unwrap().clone()
    }

    pub fn register(&mut self, task: Task) {
        let mut tasks = self.snapshot();
        let count = tasks.entry(task.clone()).or_insert(0);
        *count += 1;
        self.tasks.lock().unwrap().insert(task, *count);
    }
}

unsafe impl Send for TaskRegistry {}

unsafe impl Sync for TaskRegistry {}

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

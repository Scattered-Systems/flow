/*
   Appellation: registry <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Task;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};


pub struct TaskRegistry {
    tasks: Arc<Mutex<HashMap<Task, usize>>>,
}

impl TaskRegistry {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn register(&mut self, task: Task) {
        let mut tasks = self.tasks.lock().unwrap();
        let count = tasks.entry(task.clone()).or_insert(0);
        *count += 1;
        self.tasks.lock().unwrap().insert(task, *count);

    }
    pub fn tasks(&self) -> HashMap<Task, usize> {
        self.tasks.lock().unwrap().clone()
    }
}
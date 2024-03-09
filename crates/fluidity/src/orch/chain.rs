/*
    Appellation: chain <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::StateFn;
use std::collections::HashMap;

pub trait TaskChain {
    type Item;

    fn build_chain(&self) -> Vec<Self::Item>;
}

pub struct Registery<T> {
    store: HashMap<String, StateFn<T>>,
}

impl<T> Registery<T> {
    pub fn chain(&self, space: impl IntoIterator<Item = String>) -> Vec<StateFn<T>> {
        space
            .into_iter()
            .map(|name| {
                if let Some(task) = self.store.get(&name) {
                    return *task;
                }
                panic!("Task not found: {}", name);
            })
            .collect()
    }

    pub fn get(&self, name: &str) -> Option<&StateFn<T>> {
        self.store.get(name)
    }

    pub fn register(&mut self, name: String, task: StateFn<T>) {
        self.store.insert(name, task);
    }
}

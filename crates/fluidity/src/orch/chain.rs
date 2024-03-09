/*
    Appellation: chain <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::StateFn;
use std::collections::HashMap;

pub trait TaskChain {
    type Item;

    fn build(&self) -> Vec<Self::Item>;
}

pub struct Registery<T> {
    store: HashMap<String, StateFn<T>>,
}

impl<T> Registery<T> {
    pub fn register(&mut self, name: String, task: StateFn<T>) {
        self.store.insert(name, task);
    }
}

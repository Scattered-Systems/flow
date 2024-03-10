/*
    Appellation: orchestrator <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{FnStateSpace, Orchestrate, State};

pub struct Orchestrator<T> {
    store: FnStateSpace<T>,
}

impl<T> Orchestrator<T> {
    pub fn new(store: FnStateSpace<T>) -> Self {
        Orchestrator { store }
    }

    pub fn execute(self, state: State<T>) -> State<T>
    where
        T: Clone,
    {
        self.store.orchestrate(state)
    }
}

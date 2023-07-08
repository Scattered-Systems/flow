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

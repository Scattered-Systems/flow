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
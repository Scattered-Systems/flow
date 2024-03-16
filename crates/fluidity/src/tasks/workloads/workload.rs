/*
   Appellation: workload <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::WorkloadStatus;
use crate::prelude::AtomicId;
use crate::tasks::{GroupName, Task, TaskId};
use std::collections::{btree_map, BTreeMap};

pub(crate) type Tasks = BTreeMap<TaskId, Task>;

pub struct Workload {
    id: AtomicId,
    group: GroupName,
    name: String,
    status: WorkloadStatus,
    tasks: Tasks,
}

impl Workload {
    pub fn new(group: GroupName, name: String) -> Self {
        Self {
            id: AtomicId::default(),
            group,
            name,
            status: WorkloadStatus::Initialized,
            tasks: Tasks::new(),
        }
    }

    pub fn entry(&mut self, index: TaskId) -> btree_map::Entry<'_, TaskId, Task> {
        self.tasks.entry(index)
    }

    pub fn get(&self, index: &TaskId) -> Option<&Task> {
        self.tasks.get(index)
    }

    pub fn get_mut(&mut self, index: &TaskId) -> Option<&mut Task> {
        self.tasks.get_mut(index)
    }

    pub fn insert(&mut self, task: Task) -> Option<Task> {
        self.tasks.insert(task.id(), task)
    }

    pub fn or_insert(&mut self, task: Task) -> &mut Task {
        self.entry(task.id()).or_insert(task)
    }

    pub fn remove(&mut self, index: &TaskId) -> Option<Task> {
        self.tasks.remove(index)
    }

    pub fn id(&self) -> AtomicId {
        self.id
    }

    pub fn group(&self) -> &GroupName {
        &self.group
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn status(&self) -> WorkloadStatus {
        self.status
    }

    pub fn tasks(&self) -> &Tasks {
        &self.tasks
    }

    pub fn set_status(&mut self, status: WorkloadStatus) {
        self.status = status;
    }
}

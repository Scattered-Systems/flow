/*
   Appellation: workload <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::WorkloadStatus;
use crate::core::prelude::AtomicId;
use crate::tasks::GroupName;
use petgraph::graph::{EdgeIndex, NodeIndex};

pub(crate) type TaskGraph<N = Task, E = Option<String>> = petgraph::prelude::DiGraph<N, E>;

pub struct Task {
    index: NodeIndex,
    module: Vec<u8>,
    name: String,
    status: WorkloadStatus,
}

pub struct Workload {
    id: AtomicId,
    group: GroupName,
    name: String,
    status: WorkloadStatus,
    tasks: TaskGraph,
}

impl Workload {
    pub fn new(group: GroupName, name: String) -> Self {
        Self {
            id: AtomicId::new(),
            group,
            name,
            status: WorkloadStatus::Initialized,
            tasks: TaskGraph::new(),
        }
    }

    pub fn register(&mut self, task: Task) -> NodeIndex {
        self.tasks.add_node(task)
    }

    pub fn link(&mut self, from: &Task, to: &Task) -> EdgeIndex {
        self.tasks.add_edge(from.index, to.index, None)
    }

    pub fn id(&self) -> AtomicId {
        self.id
    }

    pub fn group(&self) -> GroupName {
        self.group
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn status(&self) -> WorkloadStatus {
        self.status
    }

    pub fn tasks(&self) -> &TaskGraph {
        &self.tasks
    }

    pub fn set_status(&mut self, status: WorkloadStatus) {
        self.status = status;
    }
}

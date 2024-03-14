/*
   Appellation: workload <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::WorkloadStatus;
use crate::core::prelude::AtomicId;
use crate::tasks::GroupName;

pub(crate) type TaskGraph<N = Workload, E = Option<String>> = petgraph::prelude::DiGraph<N, E>;

pub struct Workload {
    id: AtomicId,
    group: GroupName,
    module: Vec<u8>,
    name: String,
    status: WorkloadStatus,
    tasks: TaskGraph<Workload>,
}

impl Workload {
    pub fn new(group: GroupName, name: String) -> Self {
        Self {
            id: AtomicId::new(),
            group,
            module: Vec::new(),
            name,
            status: WorkloadStatus::Initialized,
            tasks: TaskGraph::new(),
        }
    }

    pub fn with_module(mut self, module: Vec<u8>) -> Self {
        self.module = module;
        self
    }

    pub fn id(&self) -> AtomicId {
        self.id
    }

    pub fn group(&self) -> GroupName {
        self.group
    }

    pub fn module(&self) -> &[u8] {
        &self.module
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn status(&self) -> WorkloadStatus {
        self.status
    }

    pub fn tasks(&self) -> &TaskGraph<Workload> {
        &self.tasks
    }

    pub fn set_status(&mut self, status: WorkloadStatus) {
        self.status = status;
    }
}

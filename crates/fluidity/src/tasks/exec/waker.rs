/*
   Appellation: waker <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::tasks::TaskId;
use crossbeam::queue::ArrayQueue;
use std::sync::Arc;
use std::task::{Wake, Waker};

pub struct TaskWaker {
    queue: Arc<ArrayQueue<TaskId>>,
    task: TaskId,
}

impl TaskWaker {
    pub fn new(queue: Arc<ArrayQueue<TaskId>>, task: TaskId) -> Self {
        Self { queue, task }
    }
    /// Wake up the current task
    pub fn wakeup(&self) -> Result<(), TaskId> {
        self.queue.push(self.task)
    }

    pub fn into_waker(self) -> Waker {
        Waker::from(Arc::new(self))
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wakeup().unwrap()
    }
}

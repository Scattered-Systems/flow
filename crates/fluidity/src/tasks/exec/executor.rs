/*
   Appellation: executor <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::TaskWaker;
use crate::tasks::{Task, TaskId};
use core::task::{Context, Waker};
use crossbeam::queue::ArrayQueue;
use std::collections::BTreeMap;
use std::sync::Arc;

pub struct Executor {
    queue: Arc<ArrayQueue<TaskId>>,
    tasks: BTreeMap<TaskId, Task>,
    wakers: BTreeMap<TaskId, Waker>,
}

impl Executor {
    pub fn new(queue: Arc<ArrayQueue<TaskId>>, tasks: BTreeMap<TaskId, Task>) -> Self {
        Self {
            queue,
            tasks,
            wakers: BTreeMap::new(),
        }
    }

    pub fn task_queue(&self) -> &Arc<ArrayQueue<TaskId>> {
        &self.queue
    }

    pub fn tasks(&self) -> &BTreeMap<TaskId, Task> {
        &self.tasks
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();
        }
    }

    pub fn spawn(&mut self, task: TaskId, waker: Waker) -> Result<(), TaskId> {
        self.wakers.insert(task, waker);
        self.queue.push(task)
    }

    fn run_ready_tasks(&mut self) {
        // destructure `self` to avoid borrow checker errors
        let Self {
            queue,
            tasks,
            wakers,
        } = self;

        while let Some(task_id) = queue.pop() {
            let task = match tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue, // task no longer exists
            };
            let waker = wakers
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new(queue.clone(), task_id).into_waker());
            let mut context = Context::from_waker(waker);
            // match task.poll(&mut context) {
            //     Poll::Ready(()) => {
            //         // task done -> remove it and its cached waker
            //         tasks.remove(&task_id);
            //         waker_cache.remove(&task_id);
            //     }
            //     Poll::Pending => {}
            // }
        }
    }
}

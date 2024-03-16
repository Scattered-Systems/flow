/*
   Appellation: manager <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::core::prelude::Error;
use crate::core::signals::exit::{signal, Exit};
use crate::tasks::stores::{IntoRegistry, TaskRegistry};
use core::future::{pending, Future};
use core::pin::Pin;
use futures::future::try_join_all;
use futures::FutureExt;
use tokio::runtime::Handle;

pub struct TaskManager {
    children: Vec<TaskManager>,
    handle: Handle,
    on_exit: Exit,
    persist: Box<dyn std::any::Any + Send>, // Tasks that should be kept alive until the manager is dropped
    registry: TaskRegistry,
}

impl TaskManager {
    pub fn new(handle: Handle) -> Self {
        let (_signal, exit) = signal();
        Self {
            children: Vec::new(),
            handle,
            on_exit: exit,
            persist: Box::new(()),
            registry: TaskRegistry::new(),
        }
    }

    pub fn into_registry(self) -> TaskRegistry {
        self.registry
    }

    pub fn children(&self) -> &[TaskManager] {
        &self.children
    }

    pub fn future<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(async move {
            // let mut t1 = self.essential_failed_rx.next().fuse();
            let mut t2 = self.on_exit.clone().fuse();
            // Never end this future if there is no error because if there is no children, it must not stop
            let mut t3 = try_join_all(
                self.children
                    .iter_mut()
                    .map(|x| x.future())
                    .chain(std::iter::once(pending().boxed())),
            )
            .fuse();

            futures::select! {
                // _ = t1 => Err(Error::Other("Essential task failed.".into())),
                _ = t2 => Ok(()),
                res = t3 => Err(res.map(|_| ()).expect_err("this future never ends; qed")),
            }
        })
    }

    pub fn handle(&self) -> Handle {
        self.handle.clone()
    }

    pub fn include(&mut self, task: TaskManager) {
        self.children.push(task)
    }

    /// Set what the task manager should keep alive, can be called multiple times.
    pub fn persist<T: 'static + Send>(&mut self, to_keep_alive: T) {
        // allows this fn to safely called multiple times.
        let old = std::mem::replace(&mut self.persist, Box::new(()));
        self.persist = Box::new((to_keep_alive, old));
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new(Handle::current())
    }
}

impl IntoRegistry for TaskManager {
    fn into_registry(self) -> TaskRegistry {
        self.registry
    }
}

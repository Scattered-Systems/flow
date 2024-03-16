/*
   Appellation: manage <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Manage
//!
//!

pub use self::manager::*;

pub(crate) mod manager;

use crate::core::prelude::Error;
use core::future::Future;
use core::pin::Pin;

pub trait Manager {
    type Instance;
    type Registry;

    fn children(&self) -> &[Self::Instance];

    fn future<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;

    fn into_registry(self) -> Self::Registry;
}

#[cfg(test)]
mod tests {
    use super::TaskManager;
    use tokio::runtime::Handle;

    #[tokio::test]
    async fn test_task_manager() {
        let handle = Handle::current();
        let mut manager = TaskManager::new(handle);
        let mut child = TaskManager::default();
        child.include(TaskManager::default());
        manager.include(child);
        assert_eq!(manager.children().len(), 1);
    }
}

/*
   Appellation: manage <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Manage
//!
//!

pub use self::manager::*;

pub(crate) mod manager;

pub trait Manager {
    type Instance;
    type Registry;

    fn children(&self) -> Vec<Self::Instance>;

    fn registry(&self) -> &Self::Registry;
}

#[cfg(test)]
mod tests {
    use super::TaskManager;

    #[tokio::test]
    async fn test_task_manager() {
        let mut manager = TaskManager::new();
        let mut child = TaskManager::new();
        child.push(TaskManager::new());
        manager.push(child);
        assert_eq!(manager.children().len(), 1);
    }
}

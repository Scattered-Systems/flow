/*
   Appellation: task <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::GroupName;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Task {
    pub group: GroupName,
    pub name: &'static str,
}

impl Task {
    pub fn new(group: GroupName, name: &'static str) -> Self {
        Self { group, name }
    }
    pub fn is_default(&self) -> bool {
        self.group.is_default()
    }
}

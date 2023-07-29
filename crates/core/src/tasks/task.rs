/*
   Appellation: task <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Task {
    pub group: &'static str,
    pub name: &'static str,
}

impl Task {
    pub fn new(group: &'static str, name: &'static str) -> Self {
        Self { group, name }
    }
    pub fn is_default(&self) -> bool {
        self.group == super::DEFAULT_GROUP_NAME
    }
}

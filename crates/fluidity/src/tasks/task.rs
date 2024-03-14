/*
   Appellation: task <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::GroupName;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

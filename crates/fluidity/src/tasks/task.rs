/*
   Appellation: task <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::GroupName;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
pub struct Task {
    group: GroupName,
    name: String,
}

impl Task {
    pub fn new(group: GroupName, name: impl ToString) -> Self {
        Self {
            group,
            name: name.to_string(),
        }
    }

    pub fn group(&self) -> &str {
        self.group.as_ref()
    }

    pub fn is_default(&self) -> bool {
        self.group.is_default()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

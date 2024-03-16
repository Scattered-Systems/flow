/*
   Appellation: task <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{GroupName, TaskId};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
pub struct Task {
    id: TaskId,
    group: GroupName,
    name: String,
}

impl Task {
    pub fn new(group: GroupName, name: impl ToString) -> Self {
        Self {
            id: TaskId::new(),
            group,
            name: name.to_string(),
        }
    }

    pub fn group(&self) -> &str {
        self.group.as_ref()
    }

    pub fn id(&self) -> TaskId {
        self.id
    }

    pub fn is_default(&self) -> bool {
        self.group.is_default()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

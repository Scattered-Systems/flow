/*
   Appellation: task <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/


#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct Task {
    pub group: &'static str,
    pub name: &'static str,
}

impl Task {
    pub fn new(group: &'static str, name: &'static str) -> Self {
        Self { group, name }
    }
}
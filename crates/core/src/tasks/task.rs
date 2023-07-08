/*
   Appellation: task <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
pub struct Task {
    pub group: &'static str,
    pub name: &'static str,
}

impl Task {
    pub fn new(group: &'static str, name: &'static str) -> Self {
        Self { group, name }
    }

}
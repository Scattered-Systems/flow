/*
    Appellation: state <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use scsys::bson::{oid::ObjectId, DateTime};

pub struct SessionId {}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct State {
    pub id: ObjectId,
    pub message: String,
    pub timestamp: DateTime,
}

impl State {
    fn constructor(id: ObjectId, message: String, timestamp: DateTime) -> Self {
        Self {
            id,
            message,
            timestamp,
        }
    }
    pub fn new(message: String) -> Self {
        Self::constructor(ObjectId::new(), message, scsys::Temporal::now().into())
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new(String::new())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        assert_eq!(f(4, 2), 6)
    }
}

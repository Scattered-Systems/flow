/*
    Appellation: state <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use scsys::bson::{oid::ObjectId, DateTime};

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
    use super::State;

    #[test]
    fn test_state() {
        let actual = State::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}

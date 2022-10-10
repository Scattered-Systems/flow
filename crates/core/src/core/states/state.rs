/*
    Appellation: state <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use scsys::{BsonOid, Timestamp};

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct State {
    pub id: BsonOid,
    pub message: String,
    pub timestamp: Timestamp,
}

impl State {
    pub fn new(message: String) -> Self {
        let id = BsonOid::new();
        let timestamp = Timestamp::default();

        Self {
            id,
            message,
            timestamp,
        }
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

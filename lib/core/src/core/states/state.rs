/*
    Appellation: state <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use scsys::{Id, Temporal};

#[derive(Clone, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
pub struct State {
    pub id: Id,
    pub message: String,
    pub timestamp: i64,
}

impl State {
    fn constructor(id: Id, message: String, timestamp: i64) -> Self {
        Self {
            id,
            message,
            timestamp,
        }
    }
    pub fn new(message: String) -> Self {
        Self::constructor(Id::new_oid(), message, Temporal::now().timestamp())
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

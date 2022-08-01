/*
    Appellation: state <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct State {
    pub id: scsys::Id,
    pub message: String,
    pub timestamp: i64,
}

impl State {
    fn constructor(id: scsys::Id, message: String, timestamp: i64) -> Self {
        Self {
            id,
            message,
            timestamp,
        }
    }
    pub fn new(message: String) -> Self {
        Self::constructor(
            scsys::Id::generate_object_id(),
            message,
            scsys::Temporal::now().timestamp(),
        )
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

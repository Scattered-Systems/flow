/*
    Appellation: items <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use scsys::{bson::oid::ObjectId, Deserialize, Serialize, Temporal};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Item {
    pub address: String,
    pub id: ObjectId,
    pub key: String,
    pub appellation: String,
    pub description: String,
    pub timestamp: i64,
}

impl Item {
    fn constructor(
        address: String,
        id: ObjectId,
        key: String,
        appellation: String,
        description: String,
        timestamp: i64,
    ) -> Self {
        Self {
            address,
            id,
            key,
            appellation,
            description,
            timestamp,
        }
    }
    fn new(
        address: String,
        id: ObjectId,
        key: String,
        appellation: String,
        description: String,
    ) -> Self {
        Self::constructor(
            address,
            id,
            key,
            appellation,
            description,
            Temporal::now().timestamp(),
        )
    }
}

impl Default for Item {
    fn default() -> Self {
        Self::new(
            String::new(),
            ObjectId::new(),
            String::new(),
            String::new(),
            String::new(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_items() {
        let actual = Item::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}

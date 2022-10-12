/*
    Appellation: items <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use scsys::core::{BsonOid, Timestamp};

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Item {
    pub address: String,
    pub id: BsonOid,
    pub key: String,
    pub appellation: String,
    pub description: String,
    pub timestamp: Timestamp,
}

impl Item {
    pub fn new(
        address: String,
        id: BsonOid,
        key: String,
        appellation: String,
        description: String,
    ) -> Self {
        let timestamp = Timestamp::default();
        Self {
            address,
            id,
            key,
            appellation,
            description,
            timestamp,
        }
    }
}

impl Default for Item {
    fn default() -> Self {
        Self::new(
            String::new(),
            BsonOid::new(),
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

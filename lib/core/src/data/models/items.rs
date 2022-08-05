/*
    Appellation: items <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Item {
    pub address: String,
    pub id: scsys::ObjectId,
    pub key: String,
    pub appellation: String,
    pub description: String,
    pub timestamp: i64,
}

impl Item {
    fn constructor(
        address: String,
        id: scsys::ObjectId,
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
        id: scsys::ObjectId,
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
            scsys::Temporal::now().timestamp(),
        )
    }
}

impl Default for Item {
    fn default() -> Self {
        Self::new(
            String::new(),
            scsys::ObjectId::new(),
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

/*
    Appellation: state <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use scsys::core::{BsonOid, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
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


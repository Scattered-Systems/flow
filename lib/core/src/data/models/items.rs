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
    pub created: i64,
}

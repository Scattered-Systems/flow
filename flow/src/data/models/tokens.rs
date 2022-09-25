/*
   Appellation: tokens <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TokenModel {
    access_token: String,
    token_type: String,
    username: Option<String>,
}

impl TokenModel {}

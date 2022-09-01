/*
   Appellation: index
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use axum::{routing::get, Json, Router};
use scsys::{BsonDateTime, Dictionary, Timestamp};
use serde_json::json;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Homepage {
    pub path: String,
}

impl Homepage {
    pub fn new(path: String) -> Self {
        Self { path }
    }
    pub fn router(&self) -> Router {
        let mut router = Router::new();
        router = router.route(self.path.clone().as_str(), get(landing));
        router
    }
}

impl Default for Homepage {
    fn default() -> Self {
        Self::new("/".to_string())
    }
}

/// Define the landing endpoint
pub async fn landing() -> crate::api::AxumJson {
    let timestamp: BsonDateTime = Timestamp::now().into();
    let data = json!({ "timestamp": timestamp });
    Json(data)
}

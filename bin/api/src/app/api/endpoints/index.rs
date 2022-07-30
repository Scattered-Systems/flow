/*
    Appellation: index <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use axum;
use std::collections::HashMap;

///
pub fn create_route() -> axum::Router {
    axum::Router::new().route("/", axum::routing::get(base))
}

///
pub async fn base() -> axum::Json<serde_json::Value> {
    let mut cache: HashMap<String, String> = HashMap::new();
    let timestamp: bson::DateTime = chrono::Local::now().into();
    cache.insert(String::from("timestamp"), timestamp.to_string());
    axum::Json(serde_json::json!(cache))
}

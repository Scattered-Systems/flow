/*
    Appellation: index <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use axum::routing::get;
use std::collections::HashMap;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct IndexRouter {
    pub base: String,
}

impl IndexRouter {
    fn constructor(base: String) -> Self {
        Self { base }
    }
    pub fn new() -> Self {
        Self::constructor("/".to_string())
    }
    pub fn build(self) -> Result<axum::Router, scsys::BoxError> {
        let base_path: &str = &*self.base.clone();
        let mut router = axum::Router::new();
        router = router.route(base_path.clone(), get(base));
        Ok(router.clone())
    }
}

impl Default for IndexRouter {
    fn default() -> Self {
        Self::constructor("/".to_string())
    }
}

///
pub async fn base() -> axum::Json<serde_json::Value> {
    let mut cache: HashMap<String, String> = HashMap::new();
    let timestamp: scsys::bson::DateTime = scsys::Temporal::now().into();
    cache.insert(String::from("timestamp"), timestamp.to_string());
    axum::Json(serde_json::json!(cache))
}

/*
    Appellation: index <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use axum::routing::get;
use fluidity::prelude::Tokens;
use std::collections::HashMap;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct IndexRouter {
    pub path: String,
}

impl IndexRouter {
    pub fn new(path: String) -> Self {
        Self { path }
    }
    pub fn path(&self, ext: Some(&str)) -> String {
        if ext {
            format!("{}/{}", self.path.clone(), ext)
        } else {
            self.path.clone()
        }
    }
    pub fn router(&mut self) -> axum::Router {
        axum::Router::new()
    }
    pub fn create_path(&mut self, rp: &str) -> String {
        format!("{}/{}", self.path.clone(), rp)
    }
    pub fn build(&mut self) -> scsys::BoxResult<axum::Router> {
        let mut router = self.router();
        router = router.route(self.path.clone().as_str(), get(base));
        Ok(router.clone())
    }
}

impl Default for IndexRouter {
    fn default() -> Self {
        Self::new("/".to_string())
    }
}

///
pub async fn base() -> axum::Json<serde_json::Value> {
    let mut cache = scsys::Dictionary::new();
    let timestamp: scsys::bson::DateTime = scsys::Temporal::now().into();
    cache.insert(String::from("timestamp"), timestamp.to_string());
    axum::Json(serde_json::json!(cache))
}

pub async fn post_token() -> axum::Json<serde_json::Value> {
    let mut cache: scsys::Dictionary = scsys::Dictionary::new();

    axum::Json(serde_json::json!(cache))
}

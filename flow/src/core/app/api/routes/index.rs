/*
   Appellation: index <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::Context;

use axum::{extract::Path, routing::{get, post}, Extension, Json, Router};
use scsys::core::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Homepage(pub String);

impl Homepage {
    pub fn new(data: String) -> Self {
        Self(data)
    }
    pub fn router(&mut self) -> Router {
        Router::new()
            .route("/", get(landing))
            .route("/settings", get(settings))
            .route("/notifications/:id", get(notifications))
            .route("/auth/token/:id", post(token))
    }
}

impl Default for Homepage {
    fn default() -> Self {
        Self::new("/".to_string())
    }
}

/// Define the landing endpoint
pub async fn landing() -> Json<Value> {
    let data = json!({ "timestamp": Timestamp::default() });
    Json(data)
}

/// Implements the authorization url following the OAuth2 specification
pub async fn authorization(Path(id): Path<usize>) -> Json<Value> {
    let data = json!({ "id": id });
    Json(data)
}

/// Implements the OAuth2 token
pub async fn token(Path(id): Path<usize>) -> Json<Value> {
    let data = json!({ "id": id });
    Json(data)
}

/// Implements a notification endpoint
pub async fn notifications(Path(id): Path<usize>) -> Json<Value> {
    let data = json!({ "id": id });
    Json(data)
}

pub async fn settings(Extension(ctx): Extension<Context>) -> Json<Value> {
    Json(json!(ctx.settings))
}
/*
   Appellation: index <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AuthRouter(pub String);

impl AuthRouter {
    pub fn new(data: String) -> Self {
        Self(data)
    }
    pub fn router(&mut self) -> Router {
        Router::new()
            .route("/auth/:id", get(authorize))
            .route("/token/:id", post(token))
    }
}

impl Default for AuthRouter {
    fn default() -> Self {
        Self::new("/oauth".to_string())
    }
}


/// Implements the authorization url following the OAuth2 specification
pub async fn authorize(Path(id): Path<usize>) -> Json<Value> {
    let data = json!({ "id": id });
    Json(data)
}

/// Implements the OAuth2 token
pub async fn token(Path(id): Path<usize>) -> Json<Value> {
    let data = json!({ "id": id });
    Json(data)
}



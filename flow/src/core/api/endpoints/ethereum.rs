/*
    Appellation: ethereum <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{api::AxumJson, Context};
use axum::{extract::Path, routing::get, Extension};
use ethers::prelude::Middleware;
use scsys::{BsonDateTime, Dictionary, Timestamp};

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Web3Router {
    pub path: String,
}

impl Web3Router {
    pub fn new(path: String) -> Self {
        Self {
            path: path.to_string(),
        }
    }
    pub fn from<T: std::string::ToString>(path: T) -> Self {
        Self::new(path.to_string())
    }
    pub fn endpoint(&self, prefix: &str) -> String {
        format!("{}{}", self.path.clone(), prefix)
    }
    pub fn router(&self) -> axum::Router {
        let mut router = axum::Router::new();
        router = router.route(
            self.endpoint("/blocks/block/:block_id").as_str(),
            get(get_block),
        );
        router
    }
}

impl Default for Web3Router {
    fn default() -> Self {
        Self::from("/eth")
    }
}

/// Define the landing endpoint
pub async fn get_block(
    Path(block_id): Path<u64>,
    Extension(context): Extension<Context>,
) -> AxumJson {
    let provider = context.provider.http_provider().clone();
    let block = provider
        .get_block(block_id)
        .await
        .ok()
        .unwrap()
        .expect("Input Error");
    let data = serde_json::json!(
        {
            "block_number": block_id,
            "block": block
        }
    );
    axum::Json(data)
}

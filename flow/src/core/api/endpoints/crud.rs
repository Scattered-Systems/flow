/*
    Appellation: ethereum <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{api::AxumJson, models::Items, Context};
use axum::{extract::Path, routing::get, Extension};
use scsys::{BsonDateTime, Dictionary, Timestamp};

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CrudRouter {
    pub path: String,
}

impl CrudRouter {
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
            self.endpoint("/create/items/:name").as_str(),
            get(info).post(create),
        );
        router = router.route(self.endpoint("/info").as_str(), get(info));
        router
    }
}

impl Default for CrudRouter {
    fn default() -> Self {
        Self::from("/crud")
    }
}

pub async fn create(Extension(context): Extension<Context>, Path(name): Path<String>) -> AxumJson {
    let db = context.database;
    let item = Items::new(name, None);
    db.collection::<Items>("items")
        .await
        .insert_one(item.clone(), None)
        .await
        .expect("");
    axum::Json(serde_json::json!(item))
}

/// Define the landing endpoint
pub async fn info(Extension(context): Extension<Context>) -> AxumJson {
    let database = context.database.clone();
    let collections = database.collections(None).await;
    let data = serde_json::json!(
        {
            "database": database.name.clone(),
            "collections": collections
        }
    );
    axum::Json(data)
}

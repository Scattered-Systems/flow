/*
    Appellation: database <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use mongodb::Collection;

pub type NewCollectionOptions = std::option::Option<mongodb::options::CreateCollectionOptions>;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Database {
    pub name: String,
    pub uri: String,
}

impl Database {
    pub fn new(name: String, uri: String) -> Self {
        Self { name, uri }
    }
    pub fn from_settings(settings: crate::Settings) -> Self {
        let inst = settings.clone().database;
        Self::new(inst.clone().name, inst.clone().uri)
    }
    pub async fn client(&self) -> mongodb::Client {
        mongodb::Client::with_uri_str(self.uri.clone().as_str())
            .await
            .expect("Please check the uri")
    }
    pub async fn new_collection<T>(
        &self,
        name: &str,
        options: NewCollectionOptions,
    ) -> Collection<T> {
        self.database()
            .await
            .create_collection(name, options)
            .await
            .expect("Failed to create the new collection...");
        self.collection::<T>(name).await
    }
    pub async fn collection<T>(&self, name: &str) -> Collection<T> {
        self.database().await.collection::<T>(name)
    }
    pub async fn collections(
        &self,
        filter: std::option::Option<scsys::bson::Document>,
    ) -> Vec<String> {
        self.database()
            .await
            .list_collection_names(filter)
            .await
            .expect("No collections found...")
    }
    pub async fn database(&self) -> mongodb::Database {
        self.client()
            .await
            .clone()
            .database(self.name.clone().as_str())
    }
}

impl std::fmt::Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Application is connected to the database {}", self.name)
    }
}

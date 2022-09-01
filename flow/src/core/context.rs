/*
   Appellation: context
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{caches::Cache, databases::Database, providers::Provider, Settings};

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Context {
    pub cache: Cache,
    pub database: Database,
    pub provider: Provider,
    pub settings: Settings,
}

impl Context {
    pub fn new(settings: Settings) -> Self {
        let cache = Cache::from_settings(settings.clone());
        println!("{}", cache.clone());

        let database = Database::from_settings(settings.clone());
        println!("{}", database.clone());

        let provider = Provider::from_settings(&settings);
        println!("{}", provider.clone());

        Self {
            cache,
            database,
            provider,
            settings,
        }
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Context(configuration={})", self.settings)
    }
}

/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::NetworkConfig;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NetworkContext {
    cnf: NetworkConfig,
}

impl NetworkContext {
    pub fn new(cnf: NetworkConfig) -> Self {
        Self { cnf }
    }
    pub fn settings(&self) -> NetworkConfig {
        self.cnf.clone()
    }
}

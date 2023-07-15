/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumIter, EnumString, EnumVariantNames};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NetworkConfig {
    pub max_peers: Option<usize>,
    
}


#[derive(Clone, Debug, Deserialize, Display, EnumIter, EnumString, EnumVariantNames, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OverlayConfig {
    Kademlia {
        k: usize,
        alpha: usize,
        beta: usize,
        max_peers: Option<usize>,
    },
    None,
}


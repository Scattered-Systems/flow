/*
    Appellation: config <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::context::*;

mod context;

pub struct NetworkConfig {
    pub max_peers: Option<usize>,
}


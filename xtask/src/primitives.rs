/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::types::*;

pub trait BuildPipeline {
    fn init() -> Self;
    fn handle(&self) -> BoxResult<&Self>;
    fn run(&mut self) -> BoxResult;
    fn stage(&self) -> String;
}

pub(crate) mod types {
    ///
    pub type BoxResult<T = ()> = Result<T, Box<dyn std::error::Error>>;
    ///
    pub type Bundle<T = String> = std::collections::HashMap<T, Vec<Vec<T>>>;
}

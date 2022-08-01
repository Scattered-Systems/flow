/*
    Appellation: context <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::Settings;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Context {
    pub settings: Settings,
}

impl Context {
    fn constructor(settings: Settings) -> Self {
        Self { settings }
    }
    pub fn new(settings: Settings) -> Self {
        Self::constructor(settings)
    }
}

/*
    Appellation: context <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::APISettings;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Context {
    pub settings: APISettings,
}

impl Context {
    fn constructor(settings: APISettings) -> Self {
        Self { settings }
    }
    pub fn new(settings: APISettings) -> Self {
        Self::constructor(settings)
    }
}

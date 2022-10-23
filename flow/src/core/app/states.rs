/*
    Appellation: states <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone, Debug, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum State {
    Request { message: String },
    Response { message: String },
    Idle,
}

impl State {
    pub fn new(data: &str) -> Self {
        match Self::try_from(data) {
            Ok(v) => v,
            Err(_) => panic!("{:?}", scsys::Error::default()),
        }
    }
}
impl Default for State {
    fn default() -> Self {
        Self::Idle
    }
}

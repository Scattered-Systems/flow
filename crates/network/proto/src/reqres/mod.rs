/*
    Appellation: reqres <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{codec::*, request::*, response::*};

mod codec;
mod request;
mod response;

use crate::ProtocolError;
use libp2p::request_response::{self, ProtocolSupport};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

pub use libp2p::request_response::RequestId;

pub type ProtoBehaviour = request_response::Behaviour<ProtocolCodec>;

pub type ReqResEvent = request_response::Event<Request, Response>;

pub fn new() -> ProtoBehaviour {
    let protocols: Vec<(Proto, ProtocolSupport)> = vec![(Proto::default(), ProtocolSupport::Full)];
    request_response::Behaviour::new(protocols, request_response::Config::default())
}

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
)]
#[strum(serialize_all = "snake_case")]
pub enum Frame {
    Wasm(Vec<u8>),
    #[default]
    Error(ProtocolError),
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
)]
#[strum(serialize_all = "snake_case")]
pub enum Proto {
    #[default]
    V1,
}

impl AsRef<str> for Proto {
    fn as_ref(&self) -> &str {
        match self {
            Proto::V1 => "/flow/reqres/1.0.0",
        }
    }
}

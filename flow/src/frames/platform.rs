/*
    Appellation: platform <frame>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumIter, EnumString, EnumVariantNames};

#[cfg(any(target_family = "unix", target_family = "windows"))]
use tokio::sync::oneshot;
#[cfg(any(feature = "wasi", target_os = "wasi"))]
use tokio_wasi::sync::oneshot;

#[derive(Clone, Debug, Deserialize, Display, EnumIter, EnumString, EnumVariantNames, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, SmartDefault)]
#[repr(u8)]
#[strum(serialize_all = "snake_case")]
pub enum PlatformFrame {
    #[default]
    Empty,
    Transaction(String)
}

#[derive(Debug)]
pub enum PlatformCommand {
    Connect {
        addr: String,
        tx: oneshot::Sender<()>,
    },
}

impl PlatformCommand {
    pub fn connect(addr: String) -> (Self, oneshot::Receiver<()>) {
        let (tx, rx) = oneshot::channel::<()>();
        (Self::Connect { addr, tx }, rx)
    }
}
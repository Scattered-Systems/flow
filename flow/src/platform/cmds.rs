/*
    Appellation: cmds <platform>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use strum::Display;
#[cfg(any(target_family = "unix", target_family = "windows", feature = "default"))]
use tokio::sync::oneshot;
#[cfg(any(feature = "wasi", target_os = "wasi"))]
use tokio_wasi::sync::oneshot;

pub type FrameCommandTx<T = ()> = oneshot::Sender<T>;

pub type FrameCommandRx<T = ()> = oneshot::Receiver<T>;

#[derive(Clone, Debug, Display)]
pub enum PlatformCommand {
    Connect { addr: String },
    Listen { addr: String },
}

impl PlatformCommand {
    pub fn connect(addr: String) -> Self {
        Self::Connect { addr }
    }
}



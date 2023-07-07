/*
    Appellation: cmds <platform>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(any(target_family = "unix", target_family = "windows", feature = "default"))]
use tokio::sync::oneshot;
#[cfg(any(feature = "wasi", target_os = "wasi"))]
use tokio_wasi::sync::oneshot;

pub type FrameCommandTx<T = ()> = oneshot::Sender<T>;

pub type FrameCommandRx<T = ()> = oneshot::Receiver<T>;

#[derive(Debug)]
pub enum PlatformCommand {
    Connect { addr: String, tx: FrameCommandTx },
    Listen { addr: String, tx: FrameCommandTx },
}

impl PlatformCommand {
    pub fn connect(addr: String, tx: FrameCommandTx) -> Self {
        Self::Connect { addr, tx }
    }
    pub fn connect_init(addr: String) -> (Self, FrameCommandRx) {
        let (tx, rx) = oneshot::channel();
        (Self::Connect { addr, tx }, rx)
    }
}

/*
    Appellation: cmds <platform>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use strum::Display;

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

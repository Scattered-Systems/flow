/*
   Appellation: platform <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{cmds::*, events::*, frame::*};

pub mod client;

mod cmds;
mod events;
mod frame;

use fluidity::prelude::Power;
use tokio::sync::{mpsc, watch};

pub struct Platform {
    commands: mpsc::Receiver<PlatformCommand>,
    events: mpsc::Sender<PlatformEvent>,
    power: watch::Receiver<Power>,
}

/*
    Appellation: cli <binary> (fluidity-cli)
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use crate::interface::Flow;

mod cli;
mod interface;

use acme::prelude::CLISpec;

/// Run the sandbox application
#[tokio::main]
async fn main() -> Result<(), scsys::BoxError> {
    Flow::default().run()
}

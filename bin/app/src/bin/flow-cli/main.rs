/*
    Appellation: cli <binary> (fluidity-cli)
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use acme::prelude::CLISpec;
use fluidity_sdk::Flow;

#[tokio::main]
async fn main() -> scsys::BoxResult<()> {
    Flow::default().run()
}

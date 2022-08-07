/*
    Appellation: api <binary> (rs-sandbox)
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use acme::prelude::APISpec;
use fluidity_sdk::Flow;

#[tokio::main]
async fn main() -> scsys::BoxResult<()> {
    Flow::default().run().await
}

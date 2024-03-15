/*
   Appellation: flow <api>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::settings::*;

pub(crate) mod settings;

use fluidity::prelude::Result;

#[tokio::main]
async fn main() -> Result {
    println!("Hello, world!");
    Ok(())
}

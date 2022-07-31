/*
   Appellation: cli <binary>
    Creator: FL03 <jo3mccain@icloud.com>
   Description:
       Powered by ENS, Flow is designed to be your last profile empowering users to seamlessly
       control their entire digital identity.
*/
pub use crate::{app::*, core::*};

mod app;
mod core;

use acme::flavors::CLISpec;

fn main() -> Result<(), scsys::BoxError> {
    let application = IFlow::new("development".to_string(), "cli".to_string());
    println!("{}", &application);
    application.run()
}

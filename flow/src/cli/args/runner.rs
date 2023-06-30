/*
    Appellation: runner <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, Default, Deserialize, Eq, Hash, Ord, Parser, PartialEq, PartialOrd, Serialize,
)]
pub struct Runner {
    #[clap(long, short)]
    pub target: String,
}

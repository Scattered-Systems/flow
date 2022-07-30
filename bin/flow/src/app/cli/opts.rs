/*
    Appellation: opts <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(clap::Subcommand, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Opts {
    Access {
        #[clap(long, short, value_parser)]
        ensname: String,
    },
}

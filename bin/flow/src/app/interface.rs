/*
    Appellation: interface <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use clap::Parser;

#[derive(clap::ArgEnum, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum FlowArgs {
    Account,
    Vault,
    Wallet,
}

#[derive(clap::Subcommand, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum FlowSubcommands {
    Authorize {
        #[clap(default_value = "", long, required = false, value_parser)]
        username: String,
        #[clap(default_value = "", long, required = false, value_parser)]
        password: String,
    },
}

#[derive(clap::Parser, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[clap(about, author, version)]
#[clap(long_about = "")]
pub struct FlowCLI {
    #[clap(arg_enum)]
    pub args: FlowArgs,
    #[clap(subcommand)]
    pub context: FlowSubcommands,
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Flow {
    pub mode: String,
    pub name: String,
}

impl Flow {
    fn constructor(mode: String, name: String) -> Result<Self, scsys::BoxError> {
        Ok(Self { mode, name })
    }
    pub fn new(mode: String, name: String) -> Self {
        Self::constructor(mode, name).ok().unwrap()
    }
    pub fn cli(&self) -> Result<(), scsys::BoxError> {
        let data = FlowCLI::parse();
        println!("Inputs: {:#?}", &data);
        Ok(())
    }
}

impl std::fmt::Display for Flow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Application(\n\tmode={},\n\tname={}\n)",
            self.mode, self.name
        )
    }
}

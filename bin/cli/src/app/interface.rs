/*
    Appellation: interface <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::cli::{CLISpec, FlowCLI};

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
        let data = FlowCLI::run();
        println!("Inputs: {:#?}", &data);
        Ok(())
    }
}

impl CLISpec<FlowCLI> for Flow {
    fn run(&self) -> Result<(), scsys::BoxError> {
        let data = FlowCLI::run();
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

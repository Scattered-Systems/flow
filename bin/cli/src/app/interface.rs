/*
    Appellation: interface <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::cli::FlowCLI;
use acme::flavors::CLISpec;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct IFlow {
    pub mode: String,
    pub name: String,
    pub timestamp: i64,
}

impl IFlow {
    fn constructor(mode: String, name: String, timestamp: i64) -> Result<Self, scsys::BoxError> {
        Ok(Self {
            mode,
            name,
            timestamp,
        })
    }
    pub fn new(mode: String, name: String) -> Self {
        Self::constructor(mode, name, chrono::Utc::now().timestamp())
            .ok()
            .unwrap()
    }
    pub fn cli(&self) -> Result<(), scsys::BoxError> {
        let data = FlowCLI::run();
        println!("Inputs: {:#?}", &data);
        Ok(())
    }
}

impl CLISpec<FlowCLI> for IFlow {
    fn run(&self) -> Result<(), scsys::BoxError> {
        let data = FlowCLI::run();
        println!("Inputs: {:#?}", &data);
        Ok(())
    }
}

impl std::fmt::Display for IFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Flow(mode={}, name={})", self.mode, self.timestamp)
    }
}

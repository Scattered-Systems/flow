/*
    Appellation: orch <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::orchestrator::*;

pub(crate) mod orchestrator;

use crate::core::BoxResult;

pub trait Orchestrate {
    type State;

    fn execute(&mut self, state: Self::State) -> BoxResult<Self::State>;
}

#[cfg(test)]
mod tests {}
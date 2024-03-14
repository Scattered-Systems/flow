/*
   Appellation: workloads <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Workloads
//!
//!
pub use self::{status::*, workload::*};

pub(crate) mod status;
pub(crate) mod workload;

#[cfg(test)]
mod tests {}

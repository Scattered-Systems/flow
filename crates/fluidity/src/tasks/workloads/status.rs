/*
   Appellation: status <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize,),
    serde(rename_all = "lowercase", untagged)
)]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    EnumString,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    VariantNames,
)]
#[strum(serialize_all = "lowercase")]
pub enum WorkloadStatus {
    Completed,
    Executing,
    #[default]
    Initialized,
    Failed,
    Pending,
    Terminated,
}

impl WorkloadStatus {
    pub fn completed() -> Self {
        Self::Completed
    }

    pub fn executing() -> Self {
        Self::Executing
    }

    pub fn failed() -> Self {
        Self::Failed
    }

    pub fn pending() -> Self {
        Self::Pending
    }

    pub fn terminated() -> Self {
        Self::Terminated
    }
}

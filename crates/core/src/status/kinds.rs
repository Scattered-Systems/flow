/*
   Appellation: status <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
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
    SmartDefault,
    VariantNames,
)]
#[strum(serialize_all = "lowercase")]
pub enum StatusCode {
    #[default]
    Initialized(usize),
    Failed(usize),
    Pending(usize),
    Success(usize),
    Terminated(usize),
    Working(usize),
}

impl StatusCode {
    pub fn new(status: usize) -> Self {
        match status {
            0..=99 => Self::Pending(status),
            100..=149 => Self::Initialized(status),
            150..=199 => Self::Working(status),
            200..=299 => Self::Success(status),
            400..=499 => Self::Failed(status),
            _ => Self::Terminated(status),
        }
    }

    pub fn initialized(status: usize) -> Self {
        Self::Initialized(status)
    }

    pub fn failed(status: usize) -> Self {
        Self::Failed(status)
    }

    pub fn pending(status: usize) -> Self {
        Self::Pending(status)
    }

    pub fn success(status: usize) -> Self {
        Self::Success(status)
    }

    pub fn terminated(status: usize) -> Self {
        Self::Terminated(status)
    }

    pub fn status_code(&self) -> usize {
        match self {
            Self::Initialized(status) => *status,
            Self::Failed(status) => *status,
            Self::Pending(status) => *status,
            Self::Success(status) => *status,
            Self::Terminated(status) => *status,
            Self::Working(status) => *status,
        }
    }
}

impl From<usize> for StatusCode {
    fn from(status: usize) -> Self {
        Self::new(status)
    }
}

impl From<StatusCode> for usize {
    fn from(status: StatusCode) -> usize {
        status.status_code()
    }
}

/*
   Appellation: state <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # State
//!
//!
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
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub enum BinaryState {
    Invalid = 0,
    #[default]
    Valid = 1,
}

impl BinaryState {
    /// [State::Invalid] variant constructor
    pub fn invalid() -> Self {
        Self::Invalid
    }
    /// [State::Valid] variant constructor
    pub fn valid() -> Self {
        Self::Valid
    }
    ///
    pub fn update(&mut self, state: Self) {
        *self = state;
    }
}

impl From<u8> for BinaryState {
    fn from(u: u8) -> Self {
        match u % 2 {
            1 => Self::Valid,
            _ => Self::Invalid,
        }
    }
}

impl From<BinaryState> for u8 {
    fn from(s: BinaryState) -> Self {
        s as u8
    }
}
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
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    VariantNames,
)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub enum States<A = (), B = ()> {
    Invalid(A),
    Valid(B),
}

impl<A, B> States<A, B> {
    pub fn invalid(a: A) -> Self {
        Self::Invalid(a)
    }

    pub fn valid(b: B) -> Self {
        Self::Valid(b)
    }
}

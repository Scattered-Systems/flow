/*
    Appellation: execute <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::Result;

pub trait Execute {
    type Output;

    fn execute(&self) -> Self::Output;
}

pub trait TryExecute {
    type Output;

    fn try_execute(&self) -> Result<Self::Output>;
}

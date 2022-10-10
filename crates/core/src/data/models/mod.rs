/*
    Appellation: models <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{items::*, profiles::*, tokens::*, users::*};

mod items;
mod profiles;
mod tokens;
mod users;

pub trait StandardModel {
    fn created(&self) -> scsys::BsonDateTime;
    fn modified(&self) -> scsys::BsonDateTime;
}

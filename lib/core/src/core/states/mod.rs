/*
    Appellation: states <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::state::*;

mod state;

pub trait Stateful<S> {
    fn message(&self, data: String) -> String {
        format!("Message:\n{}\n\nTimestamp: {}", data, self.timestamp())
    }
    fn state(&self) -> S
        where
            Self: Sized;
    fn timestamp(&self) -> scsys::bson::DateTime {
        scsys::Temporal::now().into()
    }
}

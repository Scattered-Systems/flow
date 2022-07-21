/*
    Appellation: core
    Context: module
    Description:
        ... Summary ...
*/
pub use common::*;
pub use controls::*;
pub use credentials::*;

mod controls;
mod credentials;

mod common {
    /// Establish the aliases a user may partake in
    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub enum Aliases {
        Personal,
        Public,
        Shared,
    }

    /// Establish the aliases a user may partake in
    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub enum Identity {
        Object(bson::oid::ObjectId),
        Standard(u64),
    }

    /// Establish a consistent temporal reference point with a globally declared Timezone
    pub type AccountTimezone = chrono::Utc;
}

/*
   Appellation: primitives <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::constants::*;
// pub use self::statics::*;
pub use self::types::*;

pub(crate) mod constants {
    ///
    pub const DEFAULT_BUFFER_SIZE: usize = 1024;
}

pub(crate) mod statics {}

pub(crate) mod types {
    /// A type alias for [std::result::Result] that uses [crate::errors::Error] as the error type.
    pub type Result<T = ()> = std::result::Result<T, crate::errors::Error>;

    pub type Ts = u128;
}

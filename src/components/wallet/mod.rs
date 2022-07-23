/*
    Appellation: wallet <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use interface::*;
pub use utils::*;

mod interface;

pub trait TokenSpec<A> {
    fn appellation(&self) -> Result<(String, String), scsys::BoxError>
        where
            Self: Sized;
}

mod utils {}

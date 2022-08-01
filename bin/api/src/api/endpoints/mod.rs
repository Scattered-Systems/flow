/*
    Appellation: endpoints <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use index::*;

mod index;

///
pub trait SubRouter {
    fn address(&self) -> String;
    fn build(&self) -> Result<axum::Router, scsys::BoxError>;
    fn build_endpoint(&self, p: String) -> String {
        format!("{}/{}/{}", &self.address(), &self.prefix(), p)
    }
    fn prefix(&self) -> String;
}

/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use async_trait::async_trait;

#[async_trait]
pub trait AsyncHandle<T, Output = ()>: Send + Sync {
    type Error: std::error::Error + Send + Sync;

    async fn handle(&mut self, t: &T) -> Result<Output, Self::Error>;
}

pub trait Handle<T> {
    type Error: std::error::Error;

    fn handle(&mut self, t: &T) -> Result<(), Self::Error>;
}

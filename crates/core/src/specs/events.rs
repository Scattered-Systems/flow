/*
    Appellation: events <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Result;
use async_trait::async_trait;

pub trait Eventful {
    type Event;

    fn event(&self) -> &Self::Event;
}

#[async_trait]
pub trait EventHandle<T>: Send + Sync + 'static {
    async fn handle_event(&self, event: T) -> Result<()>;
}

pub trait Event: Send + Sync {}

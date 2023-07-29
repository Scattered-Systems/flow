/*
    Appellation: events <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::power::*;

pub(crate) mod power;

use crate::AsyncResult;

pub trait Event {
    fn event(&self) -> &Self;
}

pub trait Eventful {
    type Event: Event;

    fn event(&self) -> &Self::Event;
}

impl<T> Event for T
where
    T: Sized,
{
    fn event(&self) -> &Self {
        self
    }
}

#[async_trait::async_trait]
pub trait EventHandle<T: Event>: Send + Sync + 'static {
    async fn handle_event(&self, event: T) -> AsyncResult<()>;
}

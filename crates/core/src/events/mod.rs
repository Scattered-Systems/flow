/*
    Appellation: events <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::AsyncResult;

pub trait Event {
    fn event(&self) -> &Self;
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

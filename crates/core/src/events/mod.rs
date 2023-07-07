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

pub trait EventHandle<T: Event>: Send + Sync + 'static {
    fn handle_event(&self, event: T) -> AsyncResult<()>;
}

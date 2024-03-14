/*
   Appellation: stores <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Stores
//!
//!
pub use self::registry::*;

pub(crate) mod registry;

pub trait IntoRegistry {
    fn into_registry(self) -> TaskRegistry;
}

impl<T> IntoRegistry for T
where
    T: Into<TaskRegistry>,
{
    fn into_registry(self) -> TaskRegistry {
        self.into()
    }
}

pub trait Entry<'a> {
    type Key;
    type Value;

    fn key(&'a self) -> &Self::Key;
    fn value(&'a self) -> &Self::Value;
}

pub trait Registry<K, V> {
    fn register(&mut self, key: K, value: V);
}

#[cfg(test)]
mod tests {}
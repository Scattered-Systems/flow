/*
    Appellation: stores <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod queue;
pub mod registry;

pub trait KeyValueStore<K, V> {
    fn get(&self, key: &K) -> Option<&V>;

    fn insert(&self, key: K, value: V) -> Option<V>;

    fn remove(&self, key: &K) -> Option<V>;
}
pub trait Entry {
    type Key;
    type Value;
}

pub trait Store {
    type Entry: Entry;

    fn get(&self, key: &<Self::Entry as Entry>::Key) -> Option<&<Self::Entry as Entry>::Value>;

    fn insert(
        &self,
        key: <Self::Entry as Entry>::Key,
        value: <Self::Entry as Entry>::Value,
    ) -> Option<<Self::Entry as Entry>::Value>;

    fn remove(&self, key: &<Self::Entry as Entry>::Key) -> Option<<Self::Entry as Entry>::Value>;
}

#[cfg(test)]
mod tests {}

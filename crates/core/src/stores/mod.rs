/*
    Appellation: stores <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod queue;

pub trait KeyValueStore<K, V> {
    fn get(&self, key: &K) -> Option<&V>;

    fn insert(&self, key: K, value: V) -> Option<V>;

    fn remove(&self, key: &K) -> Option<V>;
}

#[cfg(test)]
mod tests {}

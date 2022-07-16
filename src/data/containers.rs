/*
    Appellation: containers
    Context: 
    Description:
        ... Summary ...
*/

use std::fmt::Formatter;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Containers {
    KV(KeyValue)
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

impl KeyValue {
    fn constructor(key: String, value: String) -> Result<Self, crate::DynError> {
        Ok(Self { key, value })
    }
    pub fn new(key: String, value: String) -> Self {
        Self::constructor(key, value).ok().unwrap()
    }
    pub fn from(key: &str, value: &str) -> Self {
        Self::constructor(String::from(key), String::from(value)).ok().unwrap()
    }
}

impl std::fmt::Display for KeyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KeyValue(\n\tkey={},\n\tvalue={}\n)", self.key, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kv() {
        let (a, b) = ("test", "value");
        let left = KeyValue::new(String::from(a), String::from(b));
        let right = KeyValue::from(a, b);
        assert_eq!(&left, &right);
    }
}
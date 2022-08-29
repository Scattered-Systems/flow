/*
    Appellation: grant <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

use rand::Rng;

#[derive(Clone, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
pub struct AccessGrant {
    pub grant: String,
    pub timestamp: scsys::bson::DateTime,
}

impl AccessGrant {
    fn constructor(grant: String, timestamp: scsys::bson::DateTime) -> Self {
        Self { grant, timestamp }
    }
    pub fn generator(size: usize) -> String {
        let source = crate::BIP0039::default();
        let mut cache = Vec::<String>::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            let random_index = rng.gen_range(0..source.0.clone().len());
            cache.push(source.0[random_index].clone())
        }

        cache.join(" ")
    }
    pub fn new(grant: String) -> Self {
        Self::constructor(grant, scsys::Temporal::now().into())
    }
}

impl Default for AccessGrant {
    fn default() -> Self {
        Self::new(Self::generator(12))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_grant() {
        let actual = AccessGrant::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}

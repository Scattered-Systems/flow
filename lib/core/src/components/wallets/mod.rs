/*
    Appellation: wallets <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use utils::*;
pub use wallet::*;

mod wallet;

/// Outlines the minimum requirements for creating ERC20 tokens
pub trait IERC20 {
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn supply(&self) -> String;
}

/// Encapsulates a group of possible access grants
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Grant {
    Alt([String; 16]),
    Std([String; 12]),
}

impl Grant {
    pub fn new(size: usize) -> Self {
        todo!()
    }
    pub fn from(data: Vec<String>) -> Self {
        let len = data.clone().len();
        if len == 16 {
            let res: [String; 16] = data.clone().try_into().unwrap_or_else(|v: Vec<String>| {
                panic!("Expected a Vec of length {} but it was {}", 16, v.len())
            });

            Self::Alt(res)
        } else if len == 12 {
            let res: [String; 12] = data.clone().try_into().unwrap_or_else(|v: Vec<String>| {
                panic!("Expected a Vec of length {} but it was {}", 12, v.len())
            });

            Self::Std(res)
        } else {
            panic!("Access Grant Error: Provided data was not in the correct shape")
        }
    }
}

mod utils {
    use super::*;
    use rand::Rng;

    /// Create an Access Grant of length (size)
    pub fn generate_access_grant(size: usize) -> Grant {
        let mut rng = rand::thread_rng();
        let mut cache: Vec<String> = Vec::with_capacity(size);
        for _ in 0..size {
            let r = rng.gen::<u16>();
            cache.push(r.to_string())
        }
        Grant::from(cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passphrase_generator() {
        let actual = generate_access_grant(12);
        let expected = actual.clone();
        assert_eq!(&actual, &expected)
    }
}

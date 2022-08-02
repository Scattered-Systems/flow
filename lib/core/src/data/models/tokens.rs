/*
    Appellation: tokens <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

/// Defines the standard model for creating authentication tokens
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TokenStandard {
    pub access_type: Vec<String>,
    pub token: String,
}

impl TokenStandard {
    fn constructor(access_type: Vec<String>, token: String) -> Result<Self, scsys::BoxError> {
        Ok(Self { access_type, token })
    }
    pub fn new(access_type: Vec<String>, token: String) -> Self {
        match Self::constructor(access_type, token) {
            Ok(v) => v,
            Err(e) => panic!("Token Error: {}", e),
        }
    }
}

impl Default for TokenStandard {
    // Migrate the generative token to here
    fn default() -> Self {
        Self::new(Vec::<String>::new(), String::new())
    }
}

/// Generate a secure, single use token
pub fn generate_token(n: usize) -> String {
    let cache: String = String::new();
    cache
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        assert_eq!(f(4, 2), 6)
    }
}

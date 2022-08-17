/*
    Appellation: credentials <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
pub struct Credential {
    pub account: String,
    pub created: scsys::bson::DateTime,
    pub data: Vec<String>,
}

impl Credential {
    fn constructor(account: String, created: scsys::bson::DateTime, data: Vec<String>) -> Self {
        Self {
            account,
            created,
            data,
        }
    }

    pub fn new(account: String, data: Vec<String>) -> Self {
        Self::constructor(account, scsys::DefaultTimezone::now().into(), data)
    }

    pub fn save_to_file(&self, path: &str) -> scsys::BoxResult<Self> {
        crate::save_to_file(self.clone(), path)
    }

}

impl Default for Credential {
    fn default() -> Self {
        Self::new(String::new(), Vec::<String>::new())
    }
}

#[cfg(test)]
mod tests {
    use super::Credential;

    #[test]
    fn test_default_credential() {
        let actual = Credential::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}

/*
    Appellation: credentials <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
pub struct Credential {
    pub account: String,
    pub created: i64,
    pub data: Vec<String>,
}

impl Credential {
    fn constructor(account: String, created: i64, data: Vec<String>) -> Self {
        Self {
            account,
            created,
            data,
        }
    }
    pub fn new(account: String, data: Vec<String>) -> Self {
        Self::constructor(account, chrono::Utc::now().timestamp(), data)
    }
    pub fn save_to_file(&self, file_path: &str) -> scsys::BoxResult<Self> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;
        let buf_writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(buf_writer, self)?;
        Ok(self.clone())
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

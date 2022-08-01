/*
    Appellation: accounts <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AccountModel {
    pub id: scsys::Id,
    pub key: String,

    pub active: bool,
    pub address: String,
    pub label: String,
    pub username: String,
    pub password: String,
    pub url: String,
}

impl AccountModel {
    fn constructor(
        active: bool,
        address: String,
        id: scsys::Id,
        key: String,
        label: String,
        username: String,
        password: String,
        url: String,
    ) -> Self {
        Self {
            active,
            address,
            id,
            key,
            label,
            username,
            password,
            url,
        }
    }
    pub fn new(
        active: bool,
        address: String,
        id: scsys::Id,
        key: String,
        label: String,
        username: String,
        password: String,
        url: String,
    ) -> Self {
        Self::constructor(active, address, id, key, label, username, password, url)
    }
}

impl std::fmt::Display for AccountModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Account(id={:#?}, label={:#?})", self.id, self.label)
    }
}

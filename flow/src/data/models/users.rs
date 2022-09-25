/*
   Appellation: users <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use scsys::{BsonDateTime, BsonOid};

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserModel {
    pub id: BsonOid,

    pub password: String,
    pub username: String,

    pub created: BsonDateTime,
    pub profile: Option<Profile>,
}

impl UserModel {
    pub fn new(username: String, password: String) -> Self {
        let created = scsys::Timestamp::now().into();
        let profile = Some(Profile::default());
        Self {
            id: BsonOid::new(),
            password,
            username,
            profile,
            created,
        }
    }
    pub fn object_id(&self) -> BsonOid {
        self.id
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Profile {
    pub email: String,
    pub ensname: String,
    pub name: HumanName,
}

impl Profile {
    pub fn new(email: String, ensname: String, name: HumanName) -> Self {
        Self {
            email,
            ensname,
            name,
        }
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self::new(String::new(), String::new(), HumanName::default())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum StreetAddress {
    Default(String),
    Alternate {
        street_one: String,
        street_two: String,
    },
}

impl Default for StreetAddress {
    fn default() -> Self {
        Self::Default(String::new())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PostalAddress {
    pub street: StreetAddress,
    pub city: String,
    pub zip_code: String,
}

impl PostalAddress {
    pub fn new(street: StreetAddress, city: String, zip_code: String) -> Self {
        Self {
            street,
            city,
            zip_code,
        }
    }
}

impl Default for PostalAddress {
    fn default() -> Self {
        Self::new(StreetAddress::default(), String::new(), String::new())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct HumanName {
    pub prefix: String,
    pub first: String,
    pub middle: String,
    pub last: String,
    pub suffix: String,
}

impl HumanName {
    pub fn new(
        prefix: String,
        first: String,
        middle: String,
        last: String,
        suffix: String,
    ) -> Self {
        Self {
            prefix,
            first,
            middle,
            last,
            suffix,
        }
    }
}

impl Default for HumanName {
    fn default() -> Self {
        Self::new(
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        )
    }
}

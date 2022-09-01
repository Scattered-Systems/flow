/*
    Appellation: database <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Cache {
    pub name: String,
    pub uri: String,
}

impl Cache {
    pub fn new(name: String, uri: String) -> Self {
        Self { name, uri }
    }
    pub fn from_settings(settings: crate::Settings) -> Self {
        let inst = settings.clone().cache;
        Self::new(inst.clone().name, inst.clone().uri)
    }
}

impl std::fmt::Display for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Application cache {} is connected", self.name)
    }
}

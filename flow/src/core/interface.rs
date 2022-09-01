/*
    Appellation: interface <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{api::Interface, Context, Settings};
use scsys::BoxResult;

#[derive(Clone, Copy, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum State {
    Off,
    On,
}

impl State {
    pub fn new(data: String) -> Self {
        Self::mappings()
            .get(data.as_str())
            .expect("Failed to find a match...")
            .clone()
    }
    pub fn from<T: std::string::ToString>(data: T) -> Self {
        Self::new(data.to_string().clone().to_lowercase())
    }
    pub fn mappings() -> scsys::Dictionary<Self> {
        let info = [("off".to_string(), Self::Off), ("on".to_string(), Self::On)];
        scsys::Dictionary::<Self>::from(info)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Application {
    pub context: Context,
}

impl Application {
    pub fn new() -> Self {
        println!("Initializing the application...");
        let settings = Settings::default();
        println!("{}", settings.clone());

        let context = Context::new(settings.clone());
        Self { context }
    }
    pub fn from(settings: Settings) -> Self {
        Self {
            context: Context::new(settings),
        }
    }
    pub async fn run(&self) -> BoxResult {
        let api = Interface::new(self.context.clone());

        println!("{}", api.context.settings.server.clone());
        api.logger();
        api.run().await?;
        Ok(())
    }
}

impl Default for Application {
    fn default() -> Self {
        todo!()
    }
}

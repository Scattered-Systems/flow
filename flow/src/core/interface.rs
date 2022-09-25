/*
    Appellation: interface <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{context::Context, Settings};
use self::states::State;

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Application {
    pub context: Context,
    pub state: State
}

impl Application {
    pub fn new(settings: Option<Settings>) -> Self {
        let settings = match settings {
            Some(v) => v,
            None => Settings::default() 
        };

        let context = Context::new(Some(settings));
        let state = State::new("initializing");
        Self { context, state }
    }
    pub fn set_state(&mut self, state: State) -> &Self {
        self.state = state;
        self
    }
    pub async fn run(&self) -> scsys::BoxResult {
        Ok(())
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new(None)
    }
}

pub mod states {
    use serde::{Deserialize, Serialize};
    use strum::{EnumString, EnumVariantNames};

    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        EnumString,
        EnumVariantNames,
        Eq,
        Hash,
        PartialEq,
        Serialize,
    )]
    #[strum(serialize_all = "snake_case")]
    pub enum State {
        Initializing,
        Off,
        On,
    }

    impl State {
        pub fn new(data: &str) -> Self {
            match Self::try_from(data) {
                Ok(v) => v,
                Err(_) => panic!("{:?}", scsys::Error::Default),
            }
        }
    }
    impl Default for State {
        fn default() -> Self {
            Self::new("off")
        }
    }
}

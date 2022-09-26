/*
    Appellation: interface <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{cli::CommandLineInterface, context::Context, Settings};
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
    pub fn logging(&mut self, opt: Option<String>) -> &Self {
        let logger = match self.context.settings().logger {
            Some(v) => v,
            None => {
                match opt {
                    Some(v) => scsys::prelude::Logger::new(v),
                    None => scsys::prelude::Logger::new("debug".to_string())
                }
            }
        };
        self.context.settings().logger = Some(logger);
        self
    }
    pub fn set_state(&mut self, state: State) -> &Self {
        self.state = state;
        self
    }
    pub fn cli(&self) -> scsys::BoxResult<CommandLineInterface> {
        Ok(CommandLineInterface::default())
    }
    pub async fn run(&self) -> scsys::BoxResult<&Self> {
        let _data = match self.cli() {
            Ok(v) => v,
            Err(_) => panic!("{:?}", scsys::Error::default())
        };
        Ok(self)
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

/*
    Appellation: interface <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{cli::CommandLineInterface, Context, Settings};
use scsys::{logging::Logger, BoxResult, Error};
use serde::{Deserialize, Serialize};
pub use self::states::State;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Application {
    pub context: Context,
    pub state: State
}

impl Application {
    pub fn new(settings: Option<Settings>) -> Self {
        let context = Context::new(settings);
        let state = State::new("initializing");
        Self { context, state }
    }
    pub fn logging(&mut self, opt: Option<String>) -> &Self {
        let logger = match self.context.settings().logger {
            Some(v) => v,
            None => {
                match opt {
                    Some(v) => Logger::new(v),
                    None => Logger::new("debug".to_string())
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
    pub fn cli(&self) -> BoxResult<CommandLineInterface> {
        Ok(CommandLineInterface::default())
    }
    pub async fn run(&self) -> BoxResult<&Self> {
        let _data = match self.cli() {
            Ok(v) => v,
            Err(_) => panic!("{:?}", Error::Default)
        };
        Ok(self)
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new(Some(Settings::default()))
    }
}

pub(crate) mod states {
    use serde::{Deserialize, Serialize};
    use strum::{EnumString, EnumVariantNames};

    pub enum Agency {
        Busy,
        Idle,
    }

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

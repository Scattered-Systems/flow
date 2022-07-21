/*
   Appellation: flow
   Context: binary
   Description:
       Flow is designed to support a myriad of unique workflows by establish a secure profile for
       users to manage their complete digital identity.
*/
pub use app::*;

#[tokio::main]
async fn main() -> Result<(), scsys::BoxError> {
    let application = Application::new("development".to_string(), "flow".to_string());
    println!("{}", &application);
    application.cli().expect("Application startup failed...");
    Ok(())
}

mod app {
    use clap::Parser;

    #[derive(
    clap::ArgEnum, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize,
    )]
    pub enum Args {
        Account,
        Control,
        Create,
        Discover,
    }

    #[derive(
    clap::Subcommand, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize,
    )]
    pub enum Sub {
        Authorize {
            #[clap(default_value = "", long, required = false, value_parser)]
            username: String,
            #[clap(default_value = "", long, required = false, value_parser)]
            password: String,
        },
    }

    #[derive(clap::Parser, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    #[clap(about, author, version)]
    #[clap(long_about = "")]
    pub struct CommandCenter {
        #[clap(arg_enum)]
        pub options: Args,
        #[clap(subcommand)]
        pub subs: Sub,
    }

    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Application {
        pub mode: String,
        pub name: String,
    }

    impl Application {
        fn constructor(mode: String, name: String) -> Result<Self, scsys::BoxError> {
            Ok(Self { mode, name })
        }
        pub fn new(mode: String, name: String) -> Self {
            Self::constructor(mode, name).ok().unwrap()
        }
        pub fn cli(&self) -> Result<CommandCenter, scsys::BoxError> {
            Ok(CommandCenter::parse())
        }
    }

    impl std::fmt::Display for Application {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Application(\n\tmode={},\n\tname={}\n)",
                self.mode, self.name
            )
        }
    }
}

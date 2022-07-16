/*
    Appellation: flow
    Context: binary
    Description:
        Flow is designed to support a myriad of unique workflows by establish a secure profile for
        users to manage their complete digital identity.
 */
pub use app::*;

#[tokio::main]
async fn main() {
    println!("Welcome to Flow");
}

mod app {
    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Application {
        pub mode: String,
        pub name: String,
    }

    impl Application {
        fn constructor(mode: String, name: String) -> Result<Self, fluidity::DynError> {
            Ok(Self { mode, name })
        }
        pub fn new(mode: String, name: String) -> Self {
            Self::constructor(mode, name).ok().unwrap()
        }
    }
}
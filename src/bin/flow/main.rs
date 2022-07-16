fn main() {
    println!("Welcome to Flow");
}

mod app {
    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Application {
        pub mode: String,
        pub name: String,
    }
}
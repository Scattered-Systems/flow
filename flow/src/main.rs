/*
   Appellation: Flow <binary>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...

*/
use scsys::core::BoxResult;
use tokio::net::{TcpListener, TcpStream};
pub use self::{actors::*, controllers::*, core::*, data::*};

mod actors;
mod controllers;
mod core;
mod data;

#[tokio::main]
async fn main() -> BoxResult {

    let mut app = Application::default();

    app.logging(None);
    spawn_listener().await.expect("Failed to run the spawner");
    app.run().await.expect("Application startup failed...");
    

    Ok(())
}

pub async fn sample_process(data: TcpStream) -> BoxResult {
    println!("{:?}", data);
    Ok(())
}

pub async fn spawn_listener() -> BoxResult {

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // Process each socket concurrently.
            sample_process(socket).await.expect("Failed to spawn the process");
        });
    }
}

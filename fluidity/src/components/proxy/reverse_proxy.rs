/*
   Appellation: reverse_proxy <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use scsys::prelude::BoxResult;
use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum ProxyPort<T = usize> {
    In(T),
    Out(T),
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Proxy {
    pub ports: Vec<ProxyPort>,
}

pub async fn sample_process(data: TcpStream) -> BoxResult {
    println!("{:?}", data);
    Ok(())
}

pub async fn spawn_listener(addr: &str) -> BoxResult {
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // Process each socket concurrently.
            sample_process(socket)
                .await
                .expect("Failed to spawn the provided process...");
        });
    }
}
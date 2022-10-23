/*
   Appellation: reverse_proxy <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use scsys::BoxResult;
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

pub async fn spawn_listener() -> BoxResult {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // Process each socket concurrently.
            sample_process(socket)
                .await
                .expect("Failed to spawn the process");
        });
    }
}

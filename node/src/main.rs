use fluidity::net::prelude::{Multiaddr, NetworkBuilder, Peer};
use fluidity::prelude::Power;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::fmt().compact().init();
    tracing::info!("Starting network");
    let addr: Multiaddr = "/ip4/0.0.0.0/tcp/9001".parse::<Multiaddr>().unwrap();

    let peer: Peer = Peer::default();
    let (mut client, node) = NetworkBuilder::new().with_peer(Some(peer)).build();
    client.listen(addr).await?;

    node.spawn();

    Ok(())
}

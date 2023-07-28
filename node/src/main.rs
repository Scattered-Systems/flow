
use fluidity::prelude::Power;
use fluidity::net::prelude::{NetworkStarter, Peer};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::fmt().compact().init();
    tracing::info!("Starting network");
    let (power_tx, power_rx) = tokio::sync::watch::channel(Power::On);
    let addr: libp2p::Multiaddr = "/ip4/0.0.0.0/tcp/9099".parse().unwrap();

    let peer: Peer = Peer::default();
    let mut starter = NetworkStarter::new(None, Some(peer), power_rx);
    starter.client.listen(addr).await?;

    starter.node.spawn();

    Ok(())
}


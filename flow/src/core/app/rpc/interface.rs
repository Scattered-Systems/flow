/*
    Appellation: interface <rpc>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::Context;
use scsys::BoxResult;
use tracing_subscriber::{fmt::format::FmtSpan, prelude::*};

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct RPCBackend {
    pub ctx: Context,
}

impl RPCBackend {
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }
    pub fn setup_tracing(&self) -> BoxResult<&Self> {
        let name = self.ctx.settings.application.clone().unwrap_or_default().name;
        let tracer = opentelemetry_jaeger::new_agent_pipeline()
            .with_service_name(name.as_str())
            .with_max_packet_size(2usize.pow(13))
            .install_batch(opentelemetry::runtime::Tokio)?;
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::from_default_env())
            .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .try_init()?;
        Ok(self)
    }
    pub async fn run(&self) -> BoxResult<&Self> {
        samples::sample_client(self.ctx.clone()).await?;
        Ok(self)
    }
}

pub(crate) mod samples {
    use clap::Parser;
    use scsys::BoxResult;
    use std::env;
    use std::{net::SocketAddr, time::Duration};
    use tarpc::{client, context, tokio_serde::formats::Json};
    use tokio::time::sleep;
    use tracing::Instrument;
    use tracing_subscriber::{fmt::format::FmtSpan, prelude::*};

    /// This is the service definition. It looks a lot like a trait definition.
    /// It defines one RPC, hello, which takes one arg, name, and returns a String.
    #[tarpc::service]
    pub trait World {
        /// Returns a greeting for name.
        async fn hello(name: String) -> String;
    }

    /// Initializes an OpenTelemetry tracing subscriber with a Jaeger backend.
    pub fn init_tracing(service_name: &str) -> BoxResult {
        env::set_var("OTEL_BSP_MAX_EXPORT_BATCH_SIZE", "12");

        let tracer = opentelemetry_jaeger::new_agent_pipeline()
            .with_service_name(service_name)
            .with_max_packet_size(2usize.pow(13))
            .install_batch(opentelemetry::runtime::Tokio)?;

        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::from_default_env())
            .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .try_init()?;

        Ok(())
    }

    #[derive(Parser)]
    struct Flags {
        /// Sets the server address to connect to.
        #[clap(long)]
        server_addr: SocketAddr,
        /// Sets the name to say hello to.
        #[clap(long)]
        name: String,
    }

    pub async fn sample_client(ctx: crate::Context) -> BoxResult {
        let address = ctx.settings.server.address();
        let name = "".to_string();

        let transport = tarpc::serde_transport::tcp::connect(address, Json::default);

        let client = WorldClient::new(client::Config::default(), transport.await?).spawn();

        let hello = async move {
            tokio::select! {
                hello1 = client.hello(context::current(), format!("{}1", name)) => { hello1 }
                hello2 = client.hello(context::current(), format!("{}2", name)) => { hello2 }
            }
        }
        .instrument(tracing::info_span!("Two Hellos"))
        .await;

        tracing::info!("{:?}", hello);

        sleep(Duration::from_micros(1)).await;
        opentelemetry::global::shutdown_tracer_provider();

        Ok(())
    }
}

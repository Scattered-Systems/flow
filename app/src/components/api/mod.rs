/*
    Appellation: api <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub mod endpoints;

use crate::{Context, Settings};
use acme::network::AxumServer;
use tower_http::{
    compression::CompressionLayer, propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer, trace,
};

/// Implement a standard API interface structure
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FlowAPI {
    pub address: std::net::SocketAddr,
    pub context: Context,
}

impl FlowAPI {
    fn constructor(address: std::net::SocketAddr, context: Context) -> Self {
        Self { address, context }
    }
    pub fn new(address: std::net::SocketAddr, context: Context) -> Self {
        Self::constructor(address, context)
    }
    pub async fn client(self) -> Result<axum::Router, scsys::BoxError> {
        let client = axum::Router::new()
            .merge(endpoints::IndexRouter::new().build().ok().unwrap())
            .layer(
                trace::TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
                    .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
            )
            .layer(SetSensitiveHeadersLayer::new(std::iter::once(
                http::header::AUTHORIZATION,
            )))
            .layer(CompressionLayer::new())
            .layer(PropagateHeaderLayer::new(
                http::header::HeaderName::from_static("x-request-id"),
            ))
            .layer(axum::Extension(self.context));
        Ok(client)
    }
    pub async fn server(self) -> Result<AxumServer, scsys::BoxError> {
        Ok(axum::Server::bind(&self.address.clone())
            .serve(self.client().await.ok().unwrap().into_make_service()))
    }
    pub async fn run(self) -> Result<(), scsys::BoxError> {
        self.server()
            .await
            .ok()
            .unwrap()
            .await
            .expect("Server Error");
        Ok(())
    }
}

impl Default for FlowAPI {
    fn default() -> Self {
        let settings = match Settings::new() {
            Ok(v) => v,
            Err(e) => panic!("Settings Error: {}", e),
        };

        acme::loggers::Logger::new(settings.logger.level.clone()).setup();

        let host: [u8; 4] = scsys::extract::Extractor::new('.', settings.server.host.clone())
            .extract()
            .try_into()
            .ok()
            .unwrap();
        let port = settings.server.port;

        let address: std::net::SocketAddr = std::net::SocketAddr::from((host, port));
        let context = Context::new(settings.clone());

        Self::new(address, context)
    }
}

impl std::fmt::Display for FlowAPI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "View the application locally at http://localhost:{}",
            self.context.settings.server.port
        )
    }
}

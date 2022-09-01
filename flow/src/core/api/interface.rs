/*
   Appellation: interface <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{api::endpoints, loggers::Logger, Context};
use std::net::SocketAddr;
use tower_http::{
    compression::CompressionLayer, propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer, trace,
};

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Interface {
    pub address: SocketAddr,
    pub context: Context,
}

impl Interface {
    pub fn new(context: Context) -> Self {
        Self {
            address: context.clone().settings.server.address(),
            context,
        }
    }
    pub fn logger(&self) {
        Logger::setup(&self.context.settings)
    }

    pub async fn client(&self) -> scsys::BoxResult<axum::Router> {
        let client = axum::Router::new()
            .merge(endpoints::Homepage::default().router())
            .merge(endpoints::CrudRouter::default().router())
            .merge(endpoints::Web3Router::default().router())
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
            .layer(axum::Extension(self.context.clone()));
        Ok(client)
    }

    pub async fn run(&self) -> scsys::BoxResult {
        let client = self.client().await.expect("Client error...").clone();
        let server = axum::Server::bind(&self.address.clone())
            .serve(client.into_make_service())
            .await
            .expect("Server error");
        Ok(server)
    }
}

impl std::fmt::Display for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "View the application locally at http://localhost:{}",
            self.context.settings.server.port
        )
    }
}

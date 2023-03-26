use axum::{response::IntoResponse, routing::get};
use cfg_if::cfg_if;
use std::net::SocketAddr;
use thiserror::Error;

cfg_if! {
if #[cfg(feature = "trace")]{
use axum_tracing_opentelemetry::{opentelemetry_tracing_layer, response_with_trace_layer};
}}

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Port Used")]
    ConnnectionError,
    #[error("Database Not Found")]
    DatabaseNotFound,
    #[error("Cache Server not Found")]
    CacheDatabesNotFound,
    #[error("Server creation Error")]
    AxumError(#[from] hyper::Error),
    #[error("Unknown Start Server Error")]
    Unknown,
}

pub async fn create_server(cli: crate::cli::Cli) -> Result<(), ServerError> {
    let addr: SocketAddr = cli.address.parse().unwrap();
    #[allow(unused_mut)] // this mut is used when is builded with telemetry enable.
    let mut server = axum::Router::new().route("/", get(root));
    cfg_if! {
    if #[cfg(feature = "trace")] {
    server = server
        .layer(response_with_trace_layer())
        .layer(opentelemetry_tracing_layer());
    }
    }
    log::info!("Server Started with address: {:?}", cli.address.clone());
    axum::Server::bind(&addr)
        .serve(server.into_make_service())
        .await?;
    Ok(())
}

async fn root() -> impl IntoResponse {
    "Hello, World!"
}

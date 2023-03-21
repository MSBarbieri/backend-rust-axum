use std::net::SocketAddr;

use axum::{response::IntoResponse, routing::get};
use thiserror::Error;

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
    let server = axum::Router::new().route("/", get(root));

    log::info!("Server Started with address: {:?}", cli.address.clone());
    axum::Server::bind(&addr)
        .serve(server.into_make_service())
        .await?;
    Ok(())
}

async fn root() -> impl IntoResponse {
    "Hello, World!"
}

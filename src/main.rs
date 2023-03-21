use clap::Parser;
use server::ServerError;
use thiserror::Error;

use crate::cli::{Cli, CliError};

mod cli;
mod logger;
mod server;

#[derive(Error, Debug)]
pub enum StartError {
    #[error("CliError Invalid with error: '{0}")]
    CliError(#[from] CliError),
    #[error("start server error")]
    ServerError(#[from] ServerError),
}

#[tokio::main]
async fn main() -> Result<(), StartError> {
    let cli_args = Cli::parse();
    cli_args.validate()?;
    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Into::<tracing::Level>::into(cli_args.log_level.clone()))
        .init();

    log::debug!("Cli Validated, starting server");
    server::create_server(cli_args).await?;
    Ok(())
}

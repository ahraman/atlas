pub mod app;
pub mod error;

use std::sync::Arc;

use axum::{Router, response::IntoResponse, routing::get};
use tokio::net::TcpListener;

use crate::app::{App, Config};

pub use self::error::{Error, Result};

pub async fn run() -> Result<()> {
    let _ = dotenvy::dotenv()?;

    let app = Arc::new(App::new(Config::from_env()?)?);
    let router = Router::new().route("/", get(root));

    let server_url = &app.config.server_url;
    let listener = TcpListener::bind(server_url).await?;

    Ok(axum::serve(listener, router).await?)
}

async fn root() -> impl IntoResponse {
    "Hello, world!"
}

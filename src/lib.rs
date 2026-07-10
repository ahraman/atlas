pub mod error;

use axum::{Router, response::IntoResponse, routing::get};
use tokio::net::TcpListener;

pub use self::error::{Error, Result};

pub async fn run() -> Result<()> {
    let _ = dotenvy::dotenv()?;

    let router = Router::new().route("/", get(root));

    let server_url =
        &std::env::var("SERVER_URL").map_err(|e| Error::Env("SERVER_URL".to_string(), e))?;
    let listener = TcpListener::bind(server_url).await?;

    Ok(axum::serve(listener, router).await?)
}

async fn root() -> impl IntoResponse {
    "Hello, world!"
}

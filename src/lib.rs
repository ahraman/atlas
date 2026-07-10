pub mod app;
pub mod error;
pub mod web;

use std::sync::Arc;

use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    app::{App, Config},
    web::Server,
};

pub use self::error::{Error, Result};

pub async fn run() -> Result<()> {
    let _ = dotenvy::dotenv()?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().compact())
        .with(EnvFilter::from_default_env())
        .init();

    let app = Arc::new(App::new(Config::from_env()?)?);
    Server::new().run(app).await
}

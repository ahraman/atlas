pub mod app;
pub mod error;
pub mod web;

use std::sync::Arc;

use crate::{
    app::{App, Config},
    web::Server,
};

pub use self::error::{Error, Result};

pub async fn run() -> Result<()> {
    let _ = dotenvy::dotenv()?;

    let app = Arc::new(App::new(Config::from_env()?)?);
    Server::new().run(app).await
}

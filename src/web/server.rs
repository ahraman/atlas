use std::sync::Arc;

use tokio::net::TcpListener;

use crate::{Result, app::App, web::AppState};

#[derive(Debug)]
pub struct Server;

impl Server {
    pub fn new() -> Self {
        Self
    }

    pub async fn run(self, app: Arc<App>) -> Result<()> {
        let router = self.build_router(AppState::new(app.clone()));

        let server_url = &app.config.server_url;
        let listener = TcpListener::bind(server_url).await?;
        Ok(axum::serve(listener, router).await?)
    }
}

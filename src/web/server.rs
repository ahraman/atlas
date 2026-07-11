use std::{net::SocketAddr, sync::Arc};

use axum::{ServiceExt, extract::Request};
use tokio::net::TcpListener;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;

use crate::{Result, app::App, web::AppState};

#[derive(Debug)]
pub struct Server;

impl Server {
    pub fn new() -> Self {
        Self
    }

    pub async fn run(self, app: Arc<App>) -> Result<()> {
        let router = self.build_router(AppState::new(app.clone()));
        let middleware = NormalizePathLayer::trim_trailing_slash().layer(router);
        let service =
            ServiceExt::<Request>::into_make_service_with_connect_info::<SocketAddr>(middleware);

        let server_url = &app.config.server_url;
        let listener = TcpListener::bind(server_url).await?;
        tracing::info!("listening at: {server_url}");

        Ok(axum::serve(listener, service).await?)
    }
}

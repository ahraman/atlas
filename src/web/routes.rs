use axum::{Router, response::IntoResponse, routing::get};
use tower_http::services::ServeDir;

use crate::web::AppState;

impl super::Server {
    pub(super) fn build_router(&self, state: AppState) -> Router {
        Router::new()
            .route("/", get(root))
            .nest_service("/static", ServeDir::new("static"))
            .with_state(state)
    }
}

async fn root() -> impl IntoResponse {
    "Hello, world!"
}

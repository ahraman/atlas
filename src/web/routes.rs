use askama::Template;
use axum::{
    Router, debug_handler,
    response::{Html, IntoResponse},
    routing::get,
};
use tower_http::services::ServeDir;

use crate::{
    Result,
    web::{
        AppState,
        templates::{PageInfo, PageViewTemplate, SiteInfo},
    },
};

impl super::Server {
    pub(super) fn build_router(&self, state: AppState) -> Router {
        Router::new()
            .route("/", get(root))
            .nest_service("/static", ServeDir::new("static"))
            .with_state(state)
    }
}

#[debug_handler(state = AppState)]
async fn root(AppState(app): AppState) -> Result<impl IntoResponse> {
    Ok(Html::from(
        PageViewTemplate::new(
            SiteInfo::new(&app.config.site_name),
            PageInfo::new("root", "<p>Hello, world!</p>"),
        )
        .render()?,
    ))
}

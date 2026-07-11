mod routes;
pub mod server;
pub mod templates;

use std::{ops::Deref, sync::Arc};

use axum::extract::{FromRequestParts, State};

use crate::app::App;

pub use self::server::Server;

#[derive(Debug, Clone, FromRequestParts)]
#[from_request(via(State))]
pub struct AppState(pub Arc<App>);

impl AppState {
    pub fn new(app: Arc<App>) -> Self {
        Self(app)
    }
}

impl Deref for AppState {
    type Target = App;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

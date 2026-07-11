use axum::response::{IntoResponse, Response};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Dotenvy(#[from] dotenvy::Error),
    #[error("environment variable {0} returned error: {1}")]
    Env(String, #[source] std::env::VarError),
    #[error(transparent)]
    Askama(#[from] askama::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        format!("{:?}\n\n{}", self, self).into_response()
    }
}

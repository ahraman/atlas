pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Dotenvy(#[from] dotenvy::Error),
    #[error("environment variable {0} returned error: {1}")]
    Env(String, #[source] std::env::VarError),
}

use crate::{Error, Result};

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub site_name: String,
    pub server_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            site_name: Self::var("SITE_NAME")?,
            server_url: Self::var("SERVER_URL")?,
        })
    }

    fn var(key: &str) -> Result<String> {
        std::env::var(key).map_err(|e| Error::Env(key.to_string(), e))
    }
}

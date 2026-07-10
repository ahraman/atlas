pub mod config;

pub use self::config::Config;
use crate::Result;

#[derive(Debug)]
pub struct App {
    pub config: Config,
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self { config })
    }
}

pub mod error;

pub use self::error::{Error, Result};

pub async fn run() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}

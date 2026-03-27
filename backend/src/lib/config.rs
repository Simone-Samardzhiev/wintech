use anyhow::Context;
use std::env;

/// Struct holding the configuration variables.
#[derive(Debug)]
pub struct Config {
    pub address: String,
    pub frontend_path: String,
    pub database_url: String,
}

impl Config {
    /// Function to load the configuration from environment variables.
    pub fn new() -> anyhow::Result<Self> {
        let address = env::var("ADDRESS").context("Failed to get ADDRESS")?;
        let frontend_path = env::var("FRONTEND_PATH").context("Failed to get FRONTEND_PATH")?;
        let database_url = env::var("DATABASE_URL").context("Failed to get DATABASE_URL")?;

        Ok(Self {
            address,
            frontend_path,
            database_url,
        })
    }
}

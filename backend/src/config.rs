use anyhow::Context;
use std::env;

/// Struct holding the configuration variables.
#[derive(Debug)]
pub struct Config {
    pub address: String,
    pub frontend_path: String,
}

impl Config {
    /// Function to load the configuration from environment variables.
    pub fn new() -> anyhow::Result<Self> {
        let address = env::var("ADDRESS").with_context(|| "Failed to get ADDRESS")?;
        let dist_path = env::var("DIST_PATH").with_context(|| "Failed to get DIST_PATH")?;

        Ok(Self {
            address,
            frontend_path: dist_path,
        })
    }
}

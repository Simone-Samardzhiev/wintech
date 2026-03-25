use anyhow::Context;
use std::env;

/// Struct holding the configuration variables.
#[derive(Debug)]
pub struct Config {
    pub address: String,
}

impl Config {
    /// Function to load the configuration from environment variables.
    pub fn new() -> anyhow::Result<Self> {
        let address = env::var("ADDRESS").with_context(|| "Failed to get ADDRESS")?;
        Ok(Self { address })
    }
}

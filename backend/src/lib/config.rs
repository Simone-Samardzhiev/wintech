use anyhow::Context;
use std::env;
use time::Duration;
/// Struct holding the configuration variables.
#[derive(Debug)]
pub struct Config {
    pub address: String,
    pub frontend_path: String,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_refresh_expiry: Duration,
    pub jwt_access_expiry: Duration,
}

impl Config {
    /// Function to load the configuration from environment variables.
    pub fn new() -> anyhow::Result<Self> {
        let address = env::var("ADDRESS").context("Failed to get ADDRESS")?;

        let frontend_path = env::var("FRONTEND_PATH").context("Failed to get FRONTEND_PATH")?;

        let database_url = env::var("DATABASE_URL").context("Failed to get DATABASE_URL")?;

        let jwt_secret = env::var("JWT_SECRET").context("Failed to get JWT_SECRET")?;

        let jwt_refresh_expiry = Duration::seconds(
            env::var("JWT_REFRESH_EXPIRY")
                .context("Failed to get JWT_REFRESH_EXPIRY")?
                .parse::<i64>()
                .context("Failed to parse JWT_REFRESH_EXPIRY into i64")?,
        );

        let jwt_access_expiry = Duration::seconds(
            env::var("JWT_ACCESS_EXPIRY")
                .context("Failed to get JWT_ACCESS_EXPIRY")?
                .parse::<i64>()
                .context("Failed to parse JWT_ACCESS_EXPIRY into i64")?,
        );

        Ok(Self {
            address,
            frontend_path,
            database_url,
            jwt_secret,
            jwt_access_expiry,
            jwt_refresh_expiry,
        })
    }
}

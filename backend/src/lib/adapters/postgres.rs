pub mod user;

use anyhow::Context;

// Function to connect to postgres using database url.
pub async fn connect(database_url: &str) -> anyhow::Result<sqlx::PgPool> {
    sqlx::PgPool::connect(&database_url)
        .await
        .with_context(|| "failed to connect to postgresql")
}

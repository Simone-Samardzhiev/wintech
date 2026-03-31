pub mod user;
pub mod window;

use anyhow::Context;

/// Function to connect to postgres using database url.
pub async fn connect(database_url: &str) -> anyhow::Result<sqlx::PgPool> {
    sqlx::PgPool::connect(&database_url)
        .await
        .context("failed to connect to postgresql")
}

/// Function to apply all migrations.
pub async fn run_migrations(db: &sqlx::PgPool) -> anyhow::Result<()> {
    sqlx::migrate!()
        .run(db)
        .await
        .context("failed to run migrations")
}

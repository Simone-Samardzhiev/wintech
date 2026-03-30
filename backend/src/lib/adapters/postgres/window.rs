use crate::domain::window::models::{Window, WindowError};
use anyhow::Context;
use sqlx::{PgPool, Row, query};
use uuid::Uuid;

/// Implementation of [`crate::domain::window::ports::WindowRepository`]
/// using postgres.
///
/// It is safe to clone as it just keeps a pool of connections.
#[derive(Clone)]
pub struct WindowRepository {
    pool: PgPool,
}

impl WindowRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl crate::domain::window::ports::WindowRepository for WindowRepository {
    async fn get_by_user_id(&self, id: Uuid) -> Result<Vec<Window>, WindowError> {
        let rows = query(
            r#"SELECT
                id,
                preferred_temp,
                preferred_wake_up,
                preferred_wake_up
                FROM windows
                WHERE user_id = $1
                "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch windows")?;

        let mut windows: Vec<Window> = Vec::with_capacity(rows.len());
        for row in rows {
            let window = Window::parse(row.get(0), row.get(1), row.get(2), row.get(3))?;
            windows.push(window);
        }

        Ok(windows)
    }
}

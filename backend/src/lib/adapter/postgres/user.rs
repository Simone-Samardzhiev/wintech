use crate::domain::user::models::{User, UserError};
use sqlx::PgPool;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl crate::domain::user::ports::UserRepository for UserRepository {
    async fn save_user(&self, user: User) -> Result<(), UserError> {
        let result =
            sqlx::query("INSERT INTO users (id, name, email, password) VALUES ($1, $2, $3, $4)")
                .bind(user.id)
                .bind(user.name.as_ref())
                .bind(user.email.as_ref())
                .bind(user.password)
                .execute(&self.pool)
                .await;

        let error = match result {
            Ok(_) => return Ok(()),
            Err(e) => e,
        };

        if let Some(err) = error.as_database_error() {
            if Some("23505".into()) == err.code()
                && Some("users_email_key".into()) == err.constraint()
            {
                return Err(UserError::EmailAlreadyExists(user.email.into()));
            }
        }

        Err(UserError::from(
            anyhow::anyhow!(error).context("Failed to save user"),
        ))
    }
}

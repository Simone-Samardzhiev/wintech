use crate::domain::user::models::{Token, User, UserError};
use anyhow::{Context, anyhow};
use sqlx::{Error, PgPool, Row, query};

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl crate::domain::user::ports::UserRepository for UserRepository {
    async fn save(&self, user: &User) -> Result<(), UserError> {
        let result = query("INSERT INTO users (id, name, email, password) VALUES ($1, $2, $3, $4)")
            .bind(user.id)
            .bind(user.name.as_ref())
            .bind(user.email.as_ref())
            .bind(&user.password)
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
                return Err(UserError::EmailAlreadyExists(user.email.as_ref().into()));
            }
        }

        Err(UserError::from(
            anyhow::anyhow!(error).context("Failed to save user"),
        ))
    }

    async fn get_by_email(&self, email: &str) -> Result<User, UserError> {
        let result = query("SELECT id, name, email, password FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(&self.pool)
            .await;

        match result {
            Err(Error::RowNotFound) => Err(UserError::UserNotFoundByEmail(String::from(email))),
            Err(error) => Err(UserError::from(
                anyhow!(error).context("Failed to retrieve user from database"),
            )),
            Ok(row) => {
                let id = row.try_get(0).context("Failed to retrieve user id")?;
                let name = row.try_get(1).context("Failed to retrieve user name")?;
                let email = row.try_get(2).context("Failed to retrieve user email")?;
                let password = row.try_get(3).context("Failed to retrieve user password")?;

                Ok(User::parse(id, name, email, password).context("Failed to parse user")?)
            }
        }
    }
}

pub struct TokenRepository {
    pool: PgPool,
}

impl TokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl crate::domain::user::ports::TokenRepository for TokenRepository {
    async fn save(&self, token: &Token) -> Result<(), UserError> {
        query("INSERT INTO tokens (id, type, expiry, user_id) VALUES ($1, $2::token_type, $3, $4)")
            .bind(token.id)
            .bind(token.kind.as_ref())
            .bind(token.expiry)
            .bind(token.user_id)
            .execute(&self.pool)
            .await
            .context("Failed to save token")?;

        Ok(())
    }
}

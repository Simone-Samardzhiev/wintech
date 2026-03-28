use crate::domain::user::models::UserError;
use anyhow::{Context, anyhow};
use argon2::{
    PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

/// Implementation of [`crate::domain::user::ports::PasswordHasher`]
/// using argon2 algorithm.
pub struct ArgonPasswordHasher;

impl crate::domain::user::ports::PasswordHasher for ArgonPasswordHasher {
    fn hash(&self, password: &str) -> Result<String, UserError> {
        let argon = argon2::Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        let hash = argon
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow!(e))
            .context("Failed to hash password")?;

        Ok(hash.to_string())
    }

    fn verify(&self, password: &str, hash: &str) -> Result<bool, UserError> {
        let parsed_hash = argon2::password_hash::PasswordHash::new(hash)
            .context("Failed to parse stored hash")?;

        let result = argon2::Argon2::default().verify_password(password.as_bytes(), &parsed_hash);

        match result {
            Ok(_) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(UserError::Unknown(
                anyhow!(e).context("Failed to verify password"),
            )),
        }
    }
}

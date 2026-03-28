use super::models::{Token, User, UserError};
use uuid::Uuid;

/// Provides access to persistent user data.
pub trait UserRepository: Send + Sync + 'static {
    /// Saves [`User`].
    ///
    /// # Errors
    /// [`UserError::EmailAlreadyExists`] if the email already exists.
    ///
    /// [`UserError::Unknown`] if unexpected error occurs.
    fn save(&self, user: &User) -> impl Future<Output = Result<(), UserError>> + Send;

    /// Retrieves [`User`] by email.
    /// # Returns
    /// The found [`User`].
    ///
    /// # Errors
    /// [`UserError::UserNotFoundByEmail`] if no match is found.
    ///
    /// [`UserError::Unknown`] if unexpected error occurs.
    fn get_by_email(&self, email: &str) -> impl Future<Output = Result<User, UserError>> + Send;
}

/// Provides access to persistence token data.
pub trait TokenRepository: Send + Sync + 'static {
    /// Saves [`Token`]
    ///
    /// # Errors
    /// [`UserError::Unknown`] if unexpected error occurs.
    fn save(&self, token: &Token) -> impl Future<Output = Result<(), UserError>> + Send;

    /// Deletes [`Token`] by id.
    ///
    /// # Errors
    ///
    /// [`UserError::TokenNotFoundById`] if no match is found.
    ///
    /// [`UserError::Unknown`] if unexpected error occurs.
    fn delete(&self, id: Uuid) -> impl Future<Output = Result<(), UserError>> + Send;
}

/// Provides access to password hashing.
pub trait PasswordHasher: Send + Sync + 'static {
    /// Method to hash the password.
    ///
    /// # Returns
    /// [`Ok(String)`] holding the hash.
    ///
    /// # Errors
    /// [`UserError::Unknown`] if unexpected error occurs.
    fn hash(&self, password: &str) -> Result<String, UserError>;

    /// Method to verify password with hash.
    ///
    /// # Returns
    /// [`Ok(true)`] if the password and the hash matches
    ///
    /// [`Ok(false)`] if the password and the hash does not match
    ///
    /// # Errors
    /// [`UserError::Unknown`] if unexpected error occurs.
    fn verify(&self, password: &str, hash: &str) -> Result<bool, UserError>;
}

/// Provides access to token encoding and decoding.
pub trait TokenCoder: Send + Sync + 'static {
    /// Encodes [`Token`].
    ///
    /// # Returns
    /// [`Ok(String)`] holding the hash.
    ///
    /// # Errors
    /// [`UserError::Unknown`]  if unexpected error occurs.
    fn encode(&self, token: &Token) -> Result<String, UserError>;

    /// Decodes [`Token`] from [`str`].
    ///
    /// # Returns
    /// [`Ok(Token)`] holding the token data.
    ///
    /// # Errors
    /// [`UserError::InvalidToken`] if the decoding fails.
    ///
    /// [`UserError::InvalidTokenType`] if the token type is invalid.
    ///
    /// [`UserError::Unknown`]  if unexpected error occurs.
    fn decode(&self, token: &str) -> Result<Token, UserError>;
}

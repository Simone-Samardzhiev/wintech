use super::{
    models::{LoginRequest, RegisterRequest, Token, TokenKind, Tokens, User, UserError},
    ports::{PasswordHasher, TokenCoder, TokenRepository, UserRepository},
};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

/// Provides access to user business logic.
pub trait UserService: Send + Sync + Clone + 'static {
    /// Registers a [`User`]
    ///
    /// # Errors
    /// [`UserError::EmailAlreadyExists`] if the email already exists.
    ///
    /// [`UserError::Unknown`] if unexpected error occurs.
    fn register(
        &self,
        request: RegisterRequest,
    ) -> impl Future<Output = Result<(), UserError>> + Send;

    /// Logins a user by credential.
    ///
    /// # Returns
    /// [`Ok(Tokens)`] if the credentials are correct.
    ///
    /// # Errors
    /// [`UserError::WrongCredentials`] if the credentials are incorrect.
    ///
    /// [`UserError::Unknown`] if unexpected error occurs.
    fn login(
        &self,
        request: LoginRequest,
    ) -> impl Future<Output = Result<Tokens, UserError>> + Send;

    fn refresh_session(
        &self,
        token: &Token,
    ) -> impl Future<Output = Result<Tokens, UserError>> + Send;
}

/// Default implementation of [`UserService`].
#[derive(Clone)]
pub struct DefaultUserService<UR, TR, P, T>
where
    UR: UserRepository,
    TR: TokenRepository,
    P: PasswordHasher,
    T: TokenCoder,
{
    user_repository: UR,
    token_repository: TR,
    password_hasher: P,
    token_hasher: T,
    refresh_token_expiry: Duration,
    access_token_expiry: Duration,
}

impl<UR, TR, P, T> DefaultUserService<UR, TR, P, T>
where
    UR: UserRepository,
    TR: TokenRepository,
    P: PasswordHasher,
    T: TokenCoder,
{
    pub fn new(
        user_repository: UR,
        token_repository: TR,
        password_hasher: P,
        token_hasher: T,
        refresh_token_expiry: Duration,
        access_token_expiry: Duration,
    ) -> Self {
        Self {
            user_repository,
            token_repository,
            password_hasher,
            token_hasher,
            refresh_token_expiry,
            access_token_expiry,
        }
    }
}

impl<UR, TR, P, T> UserService for DefaultUserService<UR, TR, P, T>
where
    UR: UserRepository,
    TR: TokenRepository,
    P: PasswordHasher,
    T: TokenCoder,
{
    async fn register(&self, request: RegisterRequest) -> Result<(), UserError> {
        let hash = self.password_hasher.hash(request.password.as_ref())?;

        self.user_repository
            .save(&User::new(
                Uuid::new_v4(),
                request.name,
                request.email,
                hash,
            ))
            .await
    }

    async fn login(&self, request: LoginRequest) -> Result<Tokens, UserError> {
        let user = self
            .user_repository
            .get_by_email(&request.email)
            .await
            .map_err(|e| match e {
                UserError::UserNotFoundByEmail(_) => UserError::WrongCredentials,
                _ => e,
            })?;

        let ok = self
            .password_hasher
            .verify(&request.password, &user.password)?;

        if !ok {
            return Err(UserError::WrongCredentials);
        }

        let refresh_token = Token::new(
            Uuid::new_v4(),
            TokenKind::Refresh,
            OffsetDateTime::now_utc() + self.refresh_token_expiry,
            user.id,
        );

        self.token_repository.save(&refresh_token).await?;

        Ok(Tokens::new(
            self.token_hasher.encode(&Token::new(
                Uuid::new_v4(),
                TokenKind::Access,
                OffsetDateTime::now_utc() + self.access_token_expiry,
                user.id,
            ))?,
            self.token_hasher.encode(&refresh_token)?,
        ))
    }

    async fn refresh_session(&self, token: &Token) -> Result<Tokens, UserError> {
        match token.kind {
            TokenKind::Access => return Err(UserError::InvalidTokenKind),
            TokenKind::Refresh => {}
        }

        self.token_repository
            .delete(token.id)
            .await
            .map_err(|e| match e {
                UserError::TokenNotFoundById(_) => UserError::InvalidToken,
                _ => e,
            })?;

        let refresh_token = Token::new(
            Uuid::new_v4(),
            TokenKind::Refresh,
            OffsetDateTime::now_utc() + self.refresh_token_expiry,
            token.user_id,
        );

        self.token_repository.save(&refresh_token).await?;

        Ok(Tokens::new(
            self.token_hasher.encode(&Token::new(
                Uuid::new_v4(),
                TokenKind::Access,
                OffsetDateTime::now_utc() + self.access_token_expiry,
                token.user_id,
            ))?,
            self.token_hasher.encode(&refresh_token)?,
        ))
    }
}

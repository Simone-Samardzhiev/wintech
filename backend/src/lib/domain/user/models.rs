use thiserror::Error;
use uuid::Uuid;

/// Possible errors from user validation.
#[derive(Error, Debug)]
pub enum UserValidationError {
    #[error("Invalid email address")]
    InvalidEmailAddress,

    #[error("Invalid username length (min: {min}, max: {max}, actual: {actual})")]
    InvalidUsernameLength {
        min: usize,
        max: usize,
        actual: usize,
    },

    #[error("Invalid password length (min: {min}, max: {max}, actual: {actual})")]
    InvalidPasswordLength {
        min: usize,
        max: usize,
        actual: usize,
    },

    #[error(
        "Password missing requirements (Upper: {upper}, Lower: {lower}, Special: {special}, Digit: {digit})"
    )]
    MissingCharacter {
        upper: bool,
        lower: bool,
        special: bool,
        digit: bool,
    },
}

/// User related errors.
#[derive(Error, Debug)]
pub enum UserError {
    #[error("Invalid register request")]
    InvalidRegisterRequest(Vec<UserValidationError>),

    #[error("Email address ({0}) already exists")]
    EmailAlreadyExists(String),

    #[error("User with email ({0}) does not exist")]
    UserNotFoundByEmail(String),

    #[error("Wrong credentials")]
    WrongCredentials,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Invalid token type")]
    InvalidTokenType,

    #[error("Token with id ({0}) does not exist")]
    TokenNotFoundById(Uuid),

    #[error("Unknown error")]
    Unknown(#[from] anyhow::Error),
}

/// Valid username.
#[derive(Debug)]
pub struct Username {
    name: String,
}

impl Username {
    const MIN_LENGTH: usize = 8;
    const MAX_LENGTH: usize = 128;

    pub fn parse(mut name: String) -> Result<Self, UserValidationError> {
        name = name.trim().to_string();
        let len = name.chars().count();
        if len < Self::MIN_LENGTH || len > Self::MAX_LENGTH {
            return Err(UserValidationError::InvalidUsernameLength {
                min: Self::MIN_LENGTH,
                max: Self::MAX_LENGTH,
                actual: len,
            });
        }

        Ok(Self { name })
    }
}
impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl From<Username> for String {
    fn from(username: Username) -> String {
        username.name
    }
}

/// Valid user email.
#[derive(Debug)]
pub struct UserEmail {
    email: String,
}

impl UserEmail {
    pub fn parse(mut email: String) -> Result<Self, UserValidationError> {
        email = email.trim().to_string();

        if email_address::EmailAddress::is_valid(&email) {
            return Ok(Self { email });
        }
        Err(UserValidationError::InvalidEmailAddress)
    }
}

impl AsRef<str> for UserEmail {
    fn as_ref(&self) -> &str {
        &self.email
    }
}

impl From<UserEmail> for String {
    fn from(email: UserEmail) -> String {
        email.email
    }
}

/// Valid not hashed user password.
#[derive(Debug)]
pub struct UserPassword {
    password: String,
}

impl UserPassword {
    const MIN_LENGTH: usize = 8;
    const MAX_LENGTH: usize = 24;

    pub fn parse(mut password: String) -> Result<Self, UserValidationError> {
        password = password.trim().to_string();
        let len = password.chars().count();

        if len < Self::MIN_LENGTH || len > Self::MAX_LENGTH {
            return Err(UserValidationError::InvalidPasswordLength {
                min: Self::MIN_LENGTH,
                max: Self::MAX_LENGTH,
                actual: len,
            });
        }

        let mut has_upper = false;
        let mut has_lower = false;
        let mut has_special = false;
        let mut has_digit = false;
        for c in password.chars() {
            if c.is_uppercase() {
                has_upper = true;
            }
            if c.is_lowercase() {
                has_lower = true;
            }
            if c.is_ascii_punctuation() {
                has_special = true;
            }
            if c.is_ascii_digit() {
                has_digit = true;
            }
        }

        if !has_upper || !has_lower || !has_special || !has_digit {
            return Err(UserValidationError::MissingCharacter {
                upper: has_upper,
                lower: has_lower,
                special: has_special,
                digit: has_digit,
            });
        }

        Ok(Self { password })
    }
}

impl AsRef<str> for UserPassword {
    fn as_ref(&self) -> &str {
        &self.password
    }
}

/// Valid user with hashed password.
#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub name: Username,
    pub email: UserEmail,
    pub password: String,
}

impl User {
    pub fn new(id: Uuid, name: Username, email: UserEmail, password: String) -> Self {
        Self {
            id,
            name,
            email,
            password,
        }
    }

    pub fn parse(
        id: Uuid,
        name: String,
        email: String,
        password: String,
    ) -> Result<Self, UserError> {
        let mut errs: Vec<UserValidationError> = Vec::new();

        let name = Username::parse(name).map_err(|e| errs.push(e)).ok();
        let email = UserEmail::parse(email).map_err(|e| errs.push(e)).ok();
        if !errs.is_empty() {
            return Err(UserError::InvalidRegisterRequest(errs));
        }
        Ok(Self {
            id,
            name: name.unwrap(),
            email: email.unwrap(),
            password,
        })
    }
}

/// Request used to register a user.
#[derive(Debug)]
pub struct RegisterRequest {
    pub name: Username,
    pub email: UserEmail,
    pub password: UserPassword,
}

impl RegisterRequest {
    pub fn parse(name: String, email: String, password: String) -> Result<Self, UserError> {
        let mut errs: Vec<UserValidationError> = Vec::new();

        let name = Username::parse(name).map_err(|e| errs.push(e)).ok();
        let email = UserEmail::parse(email).map_err(|e| errs.push(e)).ok();
        let password = UserPassword::parse(password).map_err(|e| errs.push(e)).ok();

        if !errs.is_empty() {
            return Err(UserError::InvalidRegisterRequest(errs));
        }
        Ok(Self {
            name: name.unwrap(),
            email: email.unwrap(),
            password: password.unwrap(),
        })
    }
}

/// Enum representing token types.
#[derive(Debug, Clone)]
pub enum TokenKind {
    Access,
    Refresh,
}

impl AsRef<str> for TokenKind {
    fn as_ref(&self) -> &str {
        match self {
            TokenKind::Access => "access",
            TokenKind::Refresh => "refresh",
        }
    }
}

impl TryFrom<&str> for TokenKind {
    type Error = UserError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "access" => Ok(TokenKind::Access),
            "refresh" => Ok(TokenKind::Refresh),
            _ => Err(UserError::InvalidTokenType),
        }
    }
}

/// Valid user token.
#[derive(Debug, Clone)]
pub struct Token {
    pub id: Uuid,
    pub kind: TokenKind,
    pub expiry: time::OffsetDateTime,
    pub user_id: Uuid,
}

impl Token {
    pub fn new(id: Uuid, kind: TokenKind, expiry: time::OffsetDateTime, user_id: Uuid) -> Self {
        Self {
            id,
            kind,
            expiry,
            user_id,
        }
    }
}

/// Request used to log in.
#[derive(Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl LoginRequest {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}

/// Struct holding refresh and access token.
#[derive(Debug)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}

impl Tokens {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            refresh_token,
        }
    }
}

use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum UserValidationError {
    #[error("Invalid email address")]
    InvalidEmailAddress,

    #[error("Invalid name length (min:{min}, max:{max}, actual:{actual})")]
    InvalidUsernameLength {
        min: usize,
        max: usize,
        actual: usize,
    },

    #[error("Invalid password length (min:{min}, max:{max}, actual:{actual})")]
    InvalidPasswordLength {
        min: usize,
        max: usize,
        actual: usize,
    },

    #[error(
        "Password missing requirements (Upper: {upper}, Lower: {lower}, Special: {special}, Number: {number})"
    )]
    MissingCharacter {
        upper: bool,
        lower: bool,
        special: bool,
        number: bool,
    },
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Invalid user")]
    ValidationError(Vec<UserValidationError>),

    #[error("Email address ({0}) already exists")]
    EmailAlreadyExists(String),

    #[error("Unknown error")]
    Unknown(#[from] anyhow::Error),
}

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

#[derive(Debug)]
pub struct UserEmail {
    email: String,
}

impl UserEmail {
    pub fn parse(email: String) -> Result<Self, UserValidationError> {
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
        let mut has_number = false;
        for c in password.chars() {
            if c.is_uppercase() {
                has_upper = true;
            }

            if c.is_lowercase() {
                has_lower = true;
            }

            if c.is_ascii_punctuation() || c.is_ascii_hexdigit() {
                has_special = true;
            }

            if c.is_ascii_digit() {
                has_number = true;
            }
        }

        if !has_upper || !has_lower || !has_special || !has_number {
            return Err(UserValidationError::MissingCharacter {
                upper: has_upper,
                lower: has_lower,
                special: has_special,
                number: has_number,
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
            return Err(UserError::ValidationError(errs));
        }
        Ok(Self {
            id,
            name: name.unwrap(),
            email: email.unwrap(),
            password,
        })
    }
}

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
            return Err(UserError::ValidationError(errs));
        }
        Ok(Self {
            name: name.unwrap(),
            email: email.unwrap(),
            password: password.unwrap(),
        })
    }
}

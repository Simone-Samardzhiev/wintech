use super::models::{Token, User, UserError};

pub trait UserRepository: Send + Sync + 'static {
    fn save(&self, user: &User) -> impl Future<Output = Result<(), UserError>> + Send;

    fn get_by_email(&self, email: &str) -> impl Future<Output = Result<User, UserError>> + Send;
}

pub trait TokenRepository: Send + Sync + 'static {
    fn save(&self, token: &Token) -> impl Future<Output = Result<(), UserError>> + Send;
}

pub trait PasswordHasher: Send + Sync + 'static {
    fn hash(&self, password: &str) -> Result<String, UserError>;

    fn verify(&self, password: &str, hash: &str) -> Result<bool, UserError>;
}

pub trait TokenHasher: Send + Sync + 'static {
    fn hash(&self, token: Token) -> Result<String, UserError>;
}

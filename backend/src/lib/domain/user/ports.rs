use super::models::{User, UserError};

pub trait UserRepository: Send + Sync + 'static {
    fn save_user(&self, request: User)
    -> impl Future<Output = Result<(), UserError>> + Send;
}

pub trait PasswordHasher: Send + Sync + 'static {
    fn hash_password(&self, password: &str) -> Result<String, UserError>;

    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, UserError>;
}

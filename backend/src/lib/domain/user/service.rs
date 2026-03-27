use super::models::{RegisterRequest, User, UserError};
use super::ports::{PasswordHasher, UserRepository};
use uuid::Uuid;

pub trait UserService: Send + Sync + 'static {
    fn register(
        &self,
        request: RegisterRequest,
    ) -> impl Future<Output = Result<(), UserError>> + Send;
}

pub struct DefaultUserService<R, P>
where
    R: UserRepository,
    P: PasswordHasher,
{
    repository: R,
    password_hasher: P,
}

impl<R, P> DefaultUserService<R, P>
where
    R: UserRepository,
    P: PasswordHasher,
{
    pub fn new(repository: R, password_hasher: P) -> Self {
        Self {
            repository,
            password_hasher,
        }
    }
}

impl<R, P> UserService for DefaultUserService<R, P>
where
    R: UserRepository,
    P: PasswordHasher,
{
    async fn register(&self, request: RegisterRequest) -> Result<(), UserError> {
        let hash = self
            .password_hasher
            .hash_password(request.password.as_ref())?;

        self.repository
            .save_user(User::new(Uuid::new_v4(), request.name, request.email, hash))
            .await
    }
}

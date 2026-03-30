use crate::domain::user::models::{Token, TokenKind};
use crate::domain::window::{
    models::{Window, WindowError},
    ports::WindowRepository,
};

pub trait WindowService: Send + Sync + 'static {
    fn get_windows(
        &self,
        token: Token,
    ) -> impl Future<Output = Result<Vec<Window>, WindowError>> + Send;
}

pub struct DefaultWindowService<R>
where
    R: WindowRepository,
{
    repository: R,
}

impl<R> DefaultWindowService<R>
where
    R: WindowRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> WindowService for DefaultWindowService<R>
where
    R: WindowRepository,
{
    async fn get_windows(&self, token: Token) -> Result<Vec<Window>, WindowError> {
        if token.kind == TokenKind::Refresh {
            return Err(WindowError::InvalidToken);
        }

        self.repository.get_by_user_id(token.user_id).await
    }
}

use crate::domain::user::models::{Token, TokenKind};
use crate::domain::window::{
    models::{Window, WindowError},
    ports::WindowRepository,
};

/// Provides access to windows business logic.
pub trait WindowService: Send + Sync + Clone + 'static {
    /// Retrieves [`Vec<Window>`] by user id.
    ///
    /// # Returns
    /// [`Ok(Vec<Windows>)`] holding the retrieved windows.
    ///
    /// # Errors
    /// [WindowError::Unknown] if unexpected error occurs.
    fn get_windows(
        &self,
        token: Token,
    ) -> impl Future<Output = Result<Vec<Window>, WindowError>> + Send;
}

/// Default implementation of [`WindowService`].
#[derive(Clone)]
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

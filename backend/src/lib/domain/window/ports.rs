use super::models::{Window, WindowError};
use uuid::Uuid;


/// Provides access to windows persistence storage.
pub trait WindowRepository: Send + Sync + 'static {
    /// Retrieves [`Vec<Window>`] by user id.
    ///
    /// # Returns
    /// [`Ok(Vec<Windows>)`] holding the retrieved windows.
    ///
    /// # Errors
    /// [WindowError::Unknown] if unexpected error occurs.
    fn get_by_user_id(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<Vec<Window>, WindowError>> + Send;
}

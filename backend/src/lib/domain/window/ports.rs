use uuid::Uuid;
use super::models::{Window, WindowError};

pub trait WindowRepository: Send + Sync + 'static {
    fn get_by_user_id(&self, id: Uuid) -> impl Future<Output = Result<Vec<Window>, WindowError>> + Send;
}

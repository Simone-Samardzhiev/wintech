use super::models::{Order, OrderError};
use uuid::Uuid;

/// Provides access to persistent order data.
pub trait OrderRepository: Send + Sync + Clone + 'static {
    /// Saves [`Order`]
    ///
    /// # Errors
    /// [`OrderError::Unknown`] if unexpected error occurs.
    fn save(&self, order: &Order) -> impl Future<Output = Result<(), OrderError>> + Send;

    /// Retrieves [`Vec<Order>`] by user id.
    ///
    /// # Returns
    /// [`Ok(Vec<Order>)`] holding the retrieved orders.
    ///
    /// # Errors
    /// [`OrderError::Unknown`] if unexpected error occurs.
    fn get_by_user_id(
        &self,
        user_id: Uuid,
    ) -> impl Future<Output = Result<Vec<Order>, OrderError>> + Send;
}

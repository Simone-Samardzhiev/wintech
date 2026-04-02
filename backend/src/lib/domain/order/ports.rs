use super::models::{Order, OrderError};

/// Provides access to persistent order data.
pub trait OrderRepository: Send + Sync + Clone + 'static {
    /// Saves [`Order`]
    ///
    /// # Errors
    /// [`OrderError::Unknown`] if unexpected error occurs.
    fn save(&self, order: &Order) -> impl Future<Output = Result<(), OrderError>> + Send;
}

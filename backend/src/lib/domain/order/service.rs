use super::models::{Order, OrderError, OrderRequest};
use super::ports::OrderRepository;
use crate::domain::user::models::{Token, TokenKind};
use time::OffsetDateTime;
use uuid::Uuid;

/// Provides access to order business logic.
pub trait OrderService: Send + Sync + Clone + 'static {
    /// Method to place a new order.
    ///
    /// # Returns
    /// [`Order`] the created order.
    ///
    /// # Errors
    /// [OrderError::Unknown] if unexpected error occurs.
    fn place_order(
        &self,
        request: OrderRequest,
        token: Token,
    ) -> impl Future<Output = Result<Order, OrderError>> + Send;
}

#[derive(Clone)]
pub struct DefaultOrderService<T>
where
    T: OrderRepository,
{
    repository: T,
}

impl<T> DefaultOrderService<T>
where
    T: OrderRepository,
{
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

impl<T> OrderService for DefaultOrderService<T>
where
    T: OrderRepository,
{
    async fn place_order(&self, request: OrderRequest, token: Token) -> Result<Order, OrderError> {
        if token.kind == TokenKind::Refresh {
            return Err(OrderError::InvalidToken);
        }

        let order = Order::new(
            Uuid::new_v4(),
            request.postal_code,
            request.city,
            request.neighborhood,
            request.street,
            request.floor_level,
            OffsetDateTime::now_utc(),
            None,
            token.user_id,
        );

        self.repository.save(&order).await?;

        Ok(order)
    }
}

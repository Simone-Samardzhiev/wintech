use crate::domain::order::models::{Order, OrderError};
use anyhow::Context;
use sqlx::{PgPool, query};

/// Implementation of [`crate::domain::order::ports::OrderRepository`]
/// using postgres.
///
/// It is safe to clone as it just keeps a pool of connections.
#[derive(Clone)]
pub struct OrderRepository {
    pool: PgPool,
}

impl OrderRepository {
    pub fn new(pool: PgPool) -> Self {
        OrderRepository { pool }
    }
}

impl crate::domain::order::ports::OrderRepository for OrderRepository {
    async fn save(&self, order: &Order) -> Result<(), OrderError> {
        query(r#"
                INSERT INTO orders
                (id, postal_code, city, neighborhood, street, floor_level, created_at, delivered_at, user_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#)
            .bind(order.id)
            .bind(order.postal_code.as_ref())
            .bind(order.city.as_ref())
            .bind(order.neighborhood.as_ref().map(|n| n.as_ref()))
            .bind(order.city.as_ref())
            .bind(order.floor_level.map(|f| i16::from(f)))
            .bind(order.created_at)
            .bind(order.delivered_at)
            .bind(order.user_id)
            .execute(&self.pool)
            .await
            .context("Failed to save order")?;

        Ok(())
    }
}

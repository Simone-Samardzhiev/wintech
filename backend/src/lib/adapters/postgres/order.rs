use crate::domain::order::models::{Order, OrderError};
use anyhow::Context;
use sqlx::{PgPool, Row, query};
use uuid::Uuid;

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

    async fn get_by_user_id(&self, user_id: Uuid) -> Result<Vec<Order>, OrderError> {
        let rows = query(r#"
                SELECT id, postal_code, city, neighborhood, street, floor_level, created_at, delivered_at
                FROM orders WHERE user_id = $1 "#)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .context("Failed to retrieve orders")?;

        let mut orders: Vec<Order> = Vec::with_capacity(rows.len());
        for row in rows {
            let order = Order::parse(
                row.get(0),
                row.get(1),
                row.get(2),
                row.get(3),
                row.get(4),
                row.get(5),
                row.get(6),
                row.get(7),
                user_id,
            )?;
            orders.push(order);
        }

        Ok(orders)
    }
}

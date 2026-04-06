use backend::adapters::postgres::order::OrderRepository as PostgresOrderRepository;
use backend::domain::order::{models::Order, ports::OrderRepository};
use sqlx::PgPool;
use uuid::Uuid;

mod fixtures;

#[sqlx::test]
#[ignore]
async fn test_order_repository_save(pool: PgPool) {
    fixtures::seed::apply_user_seed(&pool).await;

    let repository = PostgresOrderRepository::new(pool);
    repository
        .save(
            &Order::parse(
                Uuid::new_v4(),
                "10000".into(),
                "Sofia".into(),
                None,
                "Example Street".into(),
                None,
                time::OffsetDateTime::now_utc(),
                None,
                uuid::uuid!("550e8400-e29b-41d4-a716-446655440000"),
            )
            .unwrap(),
        )
        .await
        .expect("Failed to save order");
}

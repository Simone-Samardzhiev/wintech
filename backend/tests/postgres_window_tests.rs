use backend::adapters::postgres::window::WindowRepository as PostgresWindowRepository;
use backend::domain::window::ports::WindowRepository;
use sqlx::PgPool;
use uuid::{Uuid, uuid};

mod common;
mod fixtures;

#[sqlx::test]
#[ignore]
async fn test_window_repository_save(pool: PgPool) {
    fixtures::seed::apply_user_seed(&pool).await;
    fixtures::seed::apply_windows_seed(&pool).await;

    let repository = PostgresWindowRepository::new(pool);

    let expected_ids = [
        uuid!("a1111111-1111-4111-a111-111111111111"),
        uuid!("a2222222-2222-4222-a222-222222222222"),
        uuid!("a3333333-3333-4333-a333-333333333333"),
        uuid!("a4444444-4444-4444-a444-444444444444"),
    ];

    let result: Vec<Uuid> = repository
        .get_by_user_id(uuid::uuid!("550e8400-e29b-41d4-a716-446655440000"))
        .await
        .expect("Failed to fetch window")
        .iter()
        .map(|w| w.id)
        .collect();

    assert!(
        common::counter::count(&expected_ids, &result),
        "Retrieving a window IDs does not match the expected one. Expected: {:?}, Got: {:?}",
        expected_ids,
        result
    )
}

use sqlx::{PgPool, query};

pub async fn apply_user_seed(pool: &PgPool) {
    let sql = include_str!("users_seed.sql");
    query(sql)
        .execute(pool)
        .await
        .expect("Failed to execute users seed");
}

pub async fn apply_windows_seed(pool: &PgPool) {
    let sql = include_str!("windows_seed.sql");
    query(sql)
        .execute(pool)
        .await
        .expect("Failed to execute users seed");
}

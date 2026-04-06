use backend::adapters::postgres::user::TokenRepository as PostgresTokenRepository;
use backend::domain::user::ports::TokenRepository;
use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    let token_repository = PostgresTokenRepository::new(pool);
    token_repository
        .delete_expired()
        .await
        .expect("Failed to delete expired tokens");
}

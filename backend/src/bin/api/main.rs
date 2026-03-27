use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use backend::adapter::{http, password_hasher, postgres};
use backend::config::Config;
use backend::domain;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(
            fmt::layer()
                .json()
                .with_span_list(true)
                .with_current_span(true)
                .flatten_event(true),
        )
        .init();

    dotenv::dotenv().ok();
    let config = Config::new().unwrap();

    let pool = postgres::connect(&config.database_url).await.unwrap();

    let user_repository = postgres::user::UserRepository::new(pool.clone());
    let user_service = domain::user::service::DefaultUserService::new(
        user_repository,
        password_hasher::ArgonPasswordHasher,
    );

    tracing::info!(
        address = %config.address,
        fronendPath = %config.frontend_path,
        "Starting server"
    );

    let services = http::Services::new(user_service);
    let router = http::Router::new(config, services);

    router.listen().await.unwrap();
}

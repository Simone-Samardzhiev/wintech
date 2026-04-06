use backend::{
    adapters::{http, password_hashers, postgres, token_hashers},
    config::Config,
    domain,
};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

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

    dotenvy::dotenv().ok();
    let config = Config::new().unwrap();

    let pool = postgres::connect(&config.database_url).await.unwrap();
    postgres::run_migrations(&pool).await.unwrap();

    let user_repository = postgres::user::UserRepository::new(pool.clone());
    let token_repository = postgres::user::TokenRepository::new(pool.clone());
    let token_coder = token_hashers::JWTTokenCoder::new(
        config.jwt_secret.clone(),
        config.jwt_issuer.clone(),
        config.jwt_audience.clone(),
    );

    let user_service = domain::user::service::DefaultUserService::new(
        user_repository,
        token_repository,
        password_hashers::ArgonPasswordHasher,
        token_coder.clone(),
        config.jwt_refresh_expiry,
        config.jwt_access_expiry,
    );

    let window_repository = postgres::window::WindowRepository::new(pool.clone());
    let window_service = domain::window::service::DefaultWindowService::new(window_repository);

    let order_repository = postgres::order::OrderRepository::new(pool.clone());
    let order_service = domain::order::service::DefaultOrderService::new(order_repository);

    tracing::info!(
        address = %config.address,
        fronendPath = %config.frontend_path,
        "Starting server"
    );

    http::Router::new(
        config,
        user_service,
        token_coder,
        window_service,
        order_service,
    )
    .listen()
    .await
    .unwrap();
}

mod middleware;
mod user;
pub mod window;

use crate::domain::window::service::WindowService;
use crate::{
    config::Config,
    domain::{user::ports::TokenCoder, user::service::UserService},
};
use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
};
use serde::Serialize;
use std::sync::Arc;
use time::Duration;
use tower_http::services::{ServeDir, ServeFile};

pub struct AppState<U, W, T>
where
    U: UserService,
    W: WindowService,
    T: TokenCoder,
{
    pub user_service: U,
    pub window_service: W,
    pub token_coder: T,
    pub cookie_expiry: Duration,
}

impl<U, W, T> AppState<U, W, T>
where
    U: UserService,
    W: WindowService,
    T: TokenCoder,
{
    pub fn new(
        user_service: U,
        window_service: W,
        token_coder: T,
        cookie_expiry: Duration,
    ) -> Self {
        Self {
            user_service,
            window_service,
            token_coder,
            cookie_expiry,
        }
    }
}

pub struct Router<U, W, T>
where
    U: UserService,
    W: WindowService,
    T: TokenCoder,
{
    address: String,
    frontend_path: String,
    state: AppState<U, W, T>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
    details: Option<Vec<String>>,
}

impl ErrorResponse {
    pub fn new(code: u16, message: String, details: Option<Vec<String>>) -> Self {
        Self {
            code,
            message,
            details,
        }
    }
}

impl<U, W, T> Router<U, W, T>
where
    U: UserService,
    W: WindowService,
    T: TokenCoder,
{
    pub fn new(config: Config, state: AppState<U, W, T>) -> Self {
        Router {
            address: config.address,
            frontend_path: config.frontend_path,
            state,
        }
    }

    pub async fn listen(self) -> anyhow::Result<()> {
        let state = Arc::new(self.state);

        let router =
            axum::Router::new()
                .nest(
                    "/api/v1",
                    axum::Router::new()
                        .nest(
                            "/users",
                            axum::Router::new()
                                .route("/register", post(user::register))
                                .route("/login", post(user::login))
                                .route(
                                    "/refresh",
                                    post(user::refresh_session).layer(from_fn_with_state(
                                        state.clone(),
                                        middleware::jwt_middleware,
                                    )),
                                ),
                        )
                        .nest(
                            "/windows",
                            axum::Router::new()
                                .route("/", get(window::get_windows))
                                .layer(from_fn_with_state(
                                    state.clone(),
                                    middleware::jwt_middleware,
                                )),
                        ),
                )
                .fallback_service(ServeDir::new(&self.frontend_path).not_found_service(
                    ServeFile::new(std::path::Path::new(&self.frontend_path).join("index.html")),
                ))
                .with_state(state);

        let listener = tokio::net::TcpListener::bind(&self.address).await?;
        axum::serve(listener, router).await?;
        Ok(())
    }
}

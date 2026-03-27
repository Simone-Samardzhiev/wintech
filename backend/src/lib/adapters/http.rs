mod user;

use crate::config::Config;
use crate::domain::user::service::UserService;
use axum::routing::post;
use serde::Serialize;
use std::sync::Arc;
use time::Duration;
use tower_http::services::{ServeDir, ServeFile};

pub struct AppState<U>
where
    U: UserService,
{
    pub user_service: U,
    pub cookie_expiry: Duration,
}

impl<U> AppState<U>
where
    U: UserService,
{
    pub fn new(user_service: U, cookie_expiry: Duration) -> Self {
        Self {
            user_service,
            cookie_expiry,
        }
    }
}

pub struct Router<U>
where
    U: UserService,
{
    address: String,
    frontend_path: String,
    state: AppState<U>,
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

impl<U> Router<U>
where
    U: UserService,
{
    pub fn new(config: Config, state: AppState<U>) -> Self {
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
                    axum::Router::new().nest(
                        "/users",
                        axum::Router::new()
                            .route("/register", post(user::register))
                            .route("/login", post(user::login)),
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

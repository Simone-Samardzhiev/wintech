mod middleware;
mod user;
pub mod window;

use crate::domain::window::service::WindowService;
use crate::{
    config::Config,
    domain::{user::ports::TokenCoder, user::service::UserService},
};
use axum::{
    extract::FromRef,
    middleware::from_fn_with_state,
    routing::{get, post},
};
use serde::Serialize;
use time::Duration;
use tower_http::services::{ServeDir, ServeFile};

#[derive(Clone)]
struct AppState<U, T, W>
where
    U: UserService,
    T: TokenCoder,
    W: WindowService,
{
    pub user_service: U,
    pub token_coder: T,
    pub window_service: W,
    pub cookie_expiry: Duration,
}

impl<U, T, W> AppState<U, T, W>
where
    U: UserService,
    T: TokenCoder,
    W: WindowService,
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

#[derive(Clone)]
pub struct UserState<U: UserService> {
    pub user_service: U,
}

impl<U> UserState<U>
where
    U: UserService,
{
    pub fn new(user_service: U) -> Self {
        Self { user_service }
    }
}

impl<U, T, W> FromRef<AppState<U, T, W>> for UserState<U>
where
    U: UserService,
    T: TokenCoder,
    W: WindowService,
{
    fn from_ref(app_state: &AppState<U, T, W>) -> Self {
        Self::new(app_state.user_service.clone())
    }
}

#[derive(Clone)]
pub struct AuthState<T: TokenCoder> {
    pub token_coder: T,
    pub cookie_expiry: Duration,
}
impl<T> AuthState<T>
where
    T: TokenCoder,
{
    pub fn new(token_coder: T, cookie_expiry: Duration) -> Self {
        Self {
            token_coder,
            cookie_expiry,
        }
    }
}

impl<U, T, W> FromRef<AppState<U, T, W>> for AuthState<T>
where
    U: UserService,
    T: TokenCoder,
    W: WindowService,
{
    fn from_ref(app_state: &AppState<U, T, W>) -> Self {
        Self::new(app_state.token_coder.clone(), app_state.cookie_expiry)
    }
}

#[derive(Clone)]
pub struct WindowState<W: WindowService> {
    pub window_service: W,
}

impl<W> WindowState<W>
where
    W: WindowService,
{
    pub fn new(window_service: W) -> Self {
        Self { window_service }
    }
}

impl<U, T, W> FromRef<AppState<U, T, W>> for WindowState<W>
where
    U: UserService,
    T: TokenCoder,
    W: WindowService,
{
    fn from_ref(app_state: &AppState<U, T, W>) -> Self {
        Self::new(app_state.window_service.clone())
    }
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

pub struct Router<U, T, W>
where
    U: UserService,
    T: TokenCoder,
    W: WindowService,
{
    address: String,
    frontend_path: String,
    cookie_expiry: Duration,
    user_service: U,
    window_service: W,
    token_coder: T,
}

impl<U, T, W> Router<U, T, W>
where
    U: UserService,
    T: TokenCoder,
    W: WindowService,
{
    pub fn new(config: Config, user_service: U, token_coder: T, window_service: W) -> Self {
        Router {
            address: config.address,
            frontend_path: config.frontend_path,
            cookie_expiry: config.jwt_refresh_expiry,
            user_service,
            window_service,
            token_coder,
        }
    }

    pub async fn listen(self) -> anyhow::Result<()> {
        let state = AppState::new(
            self.user_service,
            self.window_service,
            self.token_coder,
            self.cookie_expiry,
        );

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

use super::AppState;
use super::ErrorResponse;
use crate::domain::user::models::UserError;
use crate::domain::user::service::UserService;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::OffsetDateTime;

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let (status, message, details) = match self {
            UserError::ValidationError(errors) => {
                let details: Vec<String> = errors.iter().map(|e| e.to_string()).collect();

                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "Invalid registration request.".to_string(),
                    Some(details),
                )
            }
            UserError::EmailAlreadyExists(email) => (
                StatusCode::CONFLICT,
                format!("Email: {} is already used.", email),
                None,
            ),
            UserError::UserNotFoundByEmail(email) => (
                StatusCode::NOT_FOUND,
                format!("User with email: {} not found.", email),
                None,
            ),
            UserError::WrongCredentials => (
                StatusCode::UNAUTHORIZED,
                "Wrong credentials.".to_string(),
                None,
            ),
            UserError::Unknown(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error.".to_string(),
                None,
            ),
        };

        (
            status,
            Json(ErrorResponse::new(status.as_u16(), message, details)),
        )
            .into_response()
    }
}

/// Expected request to log in.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

/// Function handling user registration.
#[tracing::instrument(name = "register_handler", skip(state, payload), fields(username=%payload.username, email=%payload.email))]
pub async fn register<U>(
    State(state): State<Arc<AppState<U>>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<StatusCode, UserError>
where
    U: UserService,
{
    let request = crate::domain::user::models::RegisterRequest::parse(
        payload.username.clone(),
        payload.email.clone(),
        payload.password,
    )
    .map_err(|e| {
        if let UserError::Unknown(ref error) = e {
            tracing::error!(
                error = ?error,
                username = %payload.username,
                email = %payload.email,
                "Unknow error during validating register request"
            );
        }
        e
    })?;

    state.user_service.register(request).await.map_err(|e| {
        if let UserError::Unknown(ref error) = e {
            tracing::error!(
                error = ?error,
                username = %payload.username,
                email = %payload.email,
                "Unknow error during registration of a new user"
            );
        }
        e
    })?;
    Ok(StatusCode::CREATED)
}

/// Expected request for login.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

/// Response from successful login.
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    #[serde(rename = "refreshToken")]
    refresh_token: String,
    #[serde(rename = "accessToken")]
    access_token: String,
}

impl From<crate::domain::user::models::Tokens> for LoginResponse {
    fn from(tokens: crate::domain::user::models::Tokens) -> Self {
        Self {
            refresh_token: tokens.refresh_token,
            access_token: tokens.access_token,
        }
    }
}

/// Function handling user login.
pub async fn login<U>(
    State(state): State<Arc<AppState<U>>>,
    jar: CookieJar,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, CookieJar, Json<LoginResponse>), UserError>
where
    U: UserService,
{
    let tokens = state
        .user_service
        .login(crate::domain::user::models::LoginRequest::new(
            payload.email.clone(),
            payload.password,
        ))
        .await
        .map_err(|e| {
            if let UserError::Unknown(ref error) = e {
                tracing::error!(
                    error = ?error,
                    email = %payload.email,
                    "Unknow error user login"
                );
            }
            e
        })?;

    let cookie = Cookie::build(("refresh_token", tokens.refresh_token.clone()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .expires(OffsetDateTime::now_utc() + state.cookie_expiry)
        .build();

    let update_jar = jar.add(cookie);

    Ok((
        StatusCode::CREATED,
        update_jar,
        Json(LoginResponse::from(tokens)),
    ))
}

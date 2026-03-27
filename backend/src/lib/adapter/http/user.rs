use super::ErrorResponse;
use super::Services;
use crate::domain::user::models::UserError;
use crate::domain::user::service::UserService;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use std::sync::Arc;

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
                format!("Email: {} is already used", email),
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

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[tracing::instrument(name = "register_handler", skip(state, payload), fields(username=%payload.username, email=%payload.email))]
pub async fn register<U>(
    State(state): State<Arc<Services<U>>>,
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

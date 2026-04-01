use super::ErrorResponse;
use crate::{
    adapters::http::AppState,
    domain::{
        user::{models::Token, ports::TokenCoder, service::UserService},
        window::{
            models::{Window, WindowError},
            service::WindowService,
        },
    },
};
use axum::{
    Extension, Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::sync::Arc;
use time::Time;
use uuid::Uuid;

impl IntoResponse for WindowError {
    fn into_response(self) -> Response {
        let (status, message, details) = match self {
            WindowError::InvalidWindow(errors) => {
                let details: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "Invalid window".to_string(),
                    Some(details),
                )
            }
            WindowError::InvalidToken => {
                (StatusCode::UNAUTHORIZED, "Invalid token.".to_string(), None)
            }
            WindowError::Unknown(_) => (
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

#[derive(Debug, Serialize)]
pub struct WindowResponse {
    id: Uuid,
    #[serde(rename = "preferredTemperature")]
    preferred_temperature: i16,
    #[serde(rename = "preferredWakeUpTime")]
    preferred_wakeup_time: Time,
    #[serde(rename = "preferredBedTime")]
    preferred_bedtime: Time,
}

impl From<Window> for WindowResponse {
    fn from(window: Window) -> Self {
        Self {
            id: window.id,
            preferred_temperature: window.preferred_temperature.into(),
            preferred_wakeup_time: window.preferred_wake_up_time,
            preferred_bedtime: window.preferred_bedtime,
        }
    }
}

pub async fn get_windows<U, W, T>(
    State(state): State<Arc<AppState<U, W, T>>>,
    Extension(token): Extension<Token>,
) -> Result<(StatusCode, Json<Vec<WindowResponse>>), WindowError>
where
    U: UserService,
    W: WindowService,
    T: TokenCoder,
{
    let result = state
        .window_service
        .get_windows(token)
        .await
        .map_err(|e| {
            if let WindowError::Unknown(ref error) = e {
                tracing::error!(error = ?error)
            }
            e
        })?
        .into_iter()
        .map(|window| WindowResponse::from(window))
        .collect();

    Ok((StatusCode::OK, Json(result)))
}

use super::{ErrorResponse, OrderState};
use crate::domain::order::models::Order;
use crate::domain::{
    order::{
        models::{OrderError, OrderRequest as DomainOrderRequest},
        service::OrderService,
    },
    user::models::Token,
};
use axum::response::IntoResponse;
use axum::{Extension, Json, extract::State, http::StatusCode, response::Response};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

impl IntoResponse for OrderError {
    fn into_response(self) -> Response {
        let (status, message, details) = match self {
            OrderError::InvalidOrder(errors) => {
                let details: Vec<String> = errors.into_iter().map(|e| e.to_string()).collect();
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "Invalid order.".to_string(),
                    Some(details),
                )
            }
            OrderError::InvalidOrderRequest(errors) => {
                let details: Vec<String> = errors.into_iter().map(|e| e.to_string()).collect();
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "Invalid order request.".to_string(),
                    Some(details),
                )
            }
            OrderError::InvalidToken => {
                (StatusCode::UNAUTHORIZED, "Invalid token.".to_string(), None)
            }
            OrderError::Unknown(_) => (
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

/// JSON request to place a new order.
#[derive(Deserialize, Debug)]
pub struct OrderRequest {
    #[serde(rename = "postalCode")]
    postal_code: String,
    city: String,
    neighborhood: Option<String>,
    street: String,
    #[serde(rename = "floorLevel")]
    floor_level: Option<i16>,
}

/// JSON representation of [`Order`]
#[derive(Serialize, Debug)]
pub struct OrderResponse {
    pub id: Uuid,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    pub city: String,
    pub neighborhood: Option<String>,
    pub street: String,
    #[serde(rename = "floorLevel")]
    pub floor_level: Option<i16>,
    #[serde(rename = "createdAt")]
    pub created_at: OffsetDateTime,
    #[serde(rename = "deliveredAt")]
    pub delivered_at: Option<OffsetDateTime>,
}

impl From<Order> for OrderResponse {
    fn from(value: Order) -> Self {
        let neighborhood: Option<String> =
            value.neighborhood.map(|neighborhood| neighborhood.into());

        let floor_level: Option<i16> = value.floor_level.map(|floor_level| floor_level.into());

        Self {
            id: value.id,
            postal_code: value.postal_code.into(),
            city: value.city.into(),
            neighborhood,
            street: value.street.into(),
            floor_level,
            created_at: value.created_at,
            delivered_at: value.delivered_at,
        }
    }
}

pub async fn order<O>(
    State(state): State<OrderState<O>>,
    Extension(token): Extension<Token>,
    Json(payload): Json<OrderRequest>,
) -> Result<(StatusCode, Json<OrderResponse>), OrderError>
where
    O: OrderService,
{
    let request = DomainOrderRequest::parse(
        payload.postal_code.clone(),
        payload.city.clone(),
        payload.neighborhood.clone(),
        payload.street.clone(),
        payload.floor_level.clone(),
    )
    .map_err(|e| {
        if let OrderError::Unknown(ref error) = e {
            tracing::error!(
                error=?error,
                postal_code=%payload.postal_code,
                city=%payload.city,
                neighorbood=?payload.neighborhood,
                street=%payload.street,
                floor_level=?payload.floor_level
            );
        }
        e
    })?;

    let order = state
        .order_service
        .place_order(request, token)
        .await
        .map_err(|e| {
            if let OrderError::Unknown(ref error) = e {
                tracing::error!(
                    error=?error,
                    postal_code=%payload.postal_code,
                    city=%payload.city,
                    neighorbood=?payload.neighborhood,
                    street=%payload.street,
                    floor_level=?payload.floor_level
                );
            }
            e
        })?;

    Ok((StatusCode::CREATED, Json(order.into())))
}

pub async fn get_orders<O>(
    State(state): State<OrderState<O>>,
    Extension(token): Extension<Token>,
) -> Result<(StatusCode, Json<Vec<OrderResponse>>), OrderError>
where
    O: OrderService,
{
    let result = state
        .order_service
        .get_orders(token)
        .await
        .map_err(|e| {
            if let OrderError::Unknown(ref error) = e {
                tracing::error!(
                    error=?error,
                )
            }
            e
        })?
        .into_iter()
        .map(|o| OrderResponse::from(o))
        .collect();

    Ok((StatusCode::OK, Json(result)))
}

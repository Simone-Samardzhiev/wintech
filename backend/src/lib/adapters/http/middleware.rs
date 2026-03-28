use crate::{
    adapters::http::AppState,
    domain::user::{models::UserError, ports::TokenCoder, service::UserService},
};
use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use axum_extra::extract::CookieJar;
use std::sync::Arc;

/// Function that extracts the JWT token from the [`axum::http::header::AUTHORIZATION`].
fn get_token_header(req: &Request<Body>) -> Option<String> {
    req.headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .map(|h| h.to_string())
}

/// Function that decodes [`crate::domain::user::models::Token`] from either
/// the [`axum::http::header::AUTHORIZATION`] or the cookie
/// and inserts it as extension for the next handler.
pub async fn jwt_middleware<U, T>(
    State(state): State<Arc<AppState<U, T>>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, UserError>
where
    U: UserService,
    T: TokenCoder,
{
    let token_header = get_token_header(&req);
    let cookie = CookieJar::from_headers(&req.headers())
        .get("refresh_token")
        .map(|c| c.value().to_string());

    let raw_token = token_header.or(cookie);

    match raw_token {
        None => Err(UserError::InvalidToken),
        Some(val) => {
            let token = state.token_coder.decode(&val)?;
            req.extensions_mut().insert(token);
            Ok(next.run(req).await)
        }
    }
}

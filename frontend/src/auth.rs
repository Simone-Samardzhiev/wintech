use gloo_net::{
    Error as GlooError,
    http::{Request, RequestBuilder, Response as HttpResponse},
};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Response from successfully authorization either by sending a login request
/// or refreshing the session.
#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "accessToken")]
    pub access_token: String,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Session has already expired")]
    ExpiredSession,

    #[error("Unexpected status code")]
    UnexpectedStatusCode(u16),

    #[error("Unexpected response")]
    UnexpectedResponse(GlooError),

    #[error("Failed to connect to server")]
    Network(GlooError),
}

pub async fn refresh_session() -> Result<Response, Error> {
    let response = Request::post("api/v1/users/refresh")
        .send()
        .await
        .map_err(Error::Network)?;

    match response.status() {
        201 => {
            let body = response
                .json::<Response>()
                .await
                .map_err(Error::UnexpectedResponse)?;
            Ok(body)
        }
        401 => Err(Error::ExpiredSession),
        _ => Err(Error::UnexpectedStatusCode(response.status())),
    }
}

#[derive(Debug, Error)]
pub enum AuthenticateRequestError {
    #[error("Error refreshing session")]
    AuthFailed(#[from] Error),

    #[error("Failed to connect to server")]
    Network(GlooError),
}

pub struct AuthenticateRequestResponse {
    pub http_response: HttpResponse,
    pub access_token: Option<String>,
}

impl AuthenticateRequestResponse {
    pub fn new(
        http_response: HttpResponse,
        access_token: Option<String>,
    ) -> AuthenticateRequestResponse {
        AuthenticateRequestResponse {
            access_token,
            http_response,
        }
    }
}

pub async fn authenticate_request<F>(
    token: &str,
    factory: F,
    body: Option<impl Serialize>,
) -> Result<AuthenticateRequestResponse, AuthenticateRequestError>
where
    F: Fn() -> RequestBuilder,
{
    let builder = factory().header("Authorization", &format!("Bearer {}", token));
    let request = match body {
        Some(body) => builder.json(&body).expect(""),
        None => builder.build().expect(""),
    };

    let response = request
        .send()
        .await
        .map_err(AuthenticateRequestError::Network)?;

    let auth_response = match response.status() {
        401 => refresh_session().await?,
        _ => return Ok(AuthenticateRequestResponse::new(response, None)),
    };

    Ok(AuthenticateRequestResponse::new(
        factory()
            .header(
                "Authorization",
                &format!("Bearer {}", auth_response.access_token),
            )
            .send()
            .await
            .map_err(AuthenticateRequestError::Network)?,
        Some(auth_response.access_token),
    ))
}

#[derive(Copy, Clone, Debug)]
pub struct Context {
    pub token: RwSignal<Option<String>>,
}

impl Context {
    fn new(access_token: RwSignal<Option<String>>) -> Self {
        Self {
            token: access_token,
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new(RwSignal::new(None))
    }
}

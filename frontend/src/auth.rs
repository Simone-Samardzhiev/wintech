use gloo_net::{Error as GlooError, http::Request};
use leptos::prelude::*;
use serde::Deserialize;
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

    #[error("Unexpected error")]
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

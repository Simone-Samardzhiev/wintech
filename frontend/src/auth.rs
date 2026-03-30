use gloo_net::{Error as GlooError, http::Request};
use leptos::prelude::*;
use serde::Deserialize;

/// Response from successfully authorization either by sending a login request
/// or refreshing the session.
#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "accessToken")]
    pub access_token: String,
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

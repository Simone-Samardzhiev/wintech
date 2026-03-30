use crate::auth::Error::SessionExpired;
use crate::auth::State::Pending;
use gloo_net::{Error as GlooError, http::Request};
use leptos::prelude::*;
use leptos_router::components::A;
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The session has already expired")]
    SessionExpired,

    #[error("Unknown response from the server")]
    InvalidResponse,

    #[error("Network error")]
    Network(#[from] GlooError),
}

/// Response from successfully authorization either by sending a login request
/// or refreshing the session.
#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "accessToken")]
    pub access_token: String,
}

#[derive(Debug, Clone)]
pub enum State {
    Pending,
    Error(String),
    Logged(String),
}

#[derive(Copy, Clone, Debug)]
pub struct Context {
    pub state: RwSignal<State>,
}

impl Context {
    fn new(access_token: RwSignal<State>) -> Self {
        Self {
            state: access_token,
        }
    }

    pub fn is_logged_in(&self) -> bool {
        match self.state.get() {
            State::Logged(_) => true,
            _ => false,
        }
    }
    pub fn is_error(&self) -> bool {
        match self.state.get() {
            State::Error(_) => true,
            _ => false,
        }
    }
}
pub async fn refresh_session() -> Result<Response, Error> {
    let response = Request::post("/api/v1/users/refresh").send().await?;

    match response.status() {
        201 => {
            let body = response
                .json::<Response>()
                .await
                .map_err(|_| Error::InvalidResponse)?;
            Ok(body)
        }
        401 => Err(SessionExpired),
        _ => Err(Error::InvalidResponse),
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new(RwSignal::new(Pending))
    }
}

#[component]
pub fn AuthAlert() -> impl IntoView {
    let context = use_context::<Context>().expect("Missing auth context");
    let show_alert = RwSignal::new(false);

    Effect::new(move |_| {
        if context.is_error() {
            show_alert.set(true);
        }
    });

    view! {
        <Show when=move || show_alert.get()>
            <div class="alert-overlay">
                <div class="alert-modal">
                    <h3>"Session Expired"</h3>
                    <p>"Oops! Your session appears to have expired. Please login again."</p>
                    <A
                        href="/account"
                        on:click=move |_| show_alert.set(false)
                    >
                        "Confirm"
                    </A>
                </div>
            </div>
        </Show>
    }
}

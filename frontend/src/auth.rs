use crate::auth::Error::SessionExpired;
use crate::auth::State::Pending;
use gloo_net::{
    Error as GlooError,
    http::{Request, Response as HttpResponse},
};
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
    Expired,
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

    pub fn is_expired(&self) -> bool {
        match *self.state.read() {
            State::Expired => true,
            _ => false,
        }
    }

    pub async fn refresh_session(&self) -> Result<(), Error> {
        let response = Request::post("/api/v1/users/refresh").send().await?;

        match response.status() {
            201 => {
                let body = response
                    .json::<Response>()
                    .await
                    .map_err(|_| Error::InvalidResponse)?;

                self.state.set(State::Logged(body.access_token));
                Ok(())
            }
            401 => {
                self.state.set(State::Expired);
                Err(Error::SessionExpired)
            }
            _ => Err(Error::InvalidResponse),
        }
    }

    pub async fn authenticate<F>(&self, make_request: F) -> Result<HttpResponse, Error>
    where
        F: Fn() -> Request,
    {
        let token = match self.state.get() {
            State::Logged(token) => token,
            _ => panic!("Attempt to authenticate request of non logged user?"),
        };
        let request = make_request();
        request
            .headers()
            .set("Authorization", &format!("Bearer {}", token));

        let response = request.send().await?;

        if response.status() == 401 {
            self.refresh_session().await?;
        }

        let new_token = match self.state.get() {
            State::Logged(token) => token,
            _ => Err(SessionExpired)?,
        };

        let request = make_request();
        request
            .headers()
            .set("Authorization", &format!("Bearer {}", new_token));

        match request.send().await {
            Ok(resp) => Ok(resp),
            Err(err) => Err(Error::Network(err)),
        }
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
        if context.is_expired() {
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

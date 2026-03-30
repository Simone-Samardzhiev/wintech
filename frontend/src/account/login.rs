use super::AuthMode;
use crate::{
    auth::{Context, Response},
    widgets::ProgressBar,
};
use gloo_net::http::Request;
use leptos::{
    context::use_context,
    either::Either,
    ev,
    ev::SubmitEvent,
    html::{button, div, form, h2, input, span},
    leptos_dom::log,
    prelude::*,
    task::spawn_local,
};
use serde::Serialize;

/// Struct representing the JSON request for login.
#[derive(Debug, Serialize)]
struct LoginRequest {
    email: String,
    password: String,
}

impl LoginRequest {
    fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}

/// Component used to log in.
#[component]
pub fn Login(set_mode: WriteSignal<AuthMode>) -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());

    let is_loading = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);
    let context = use_context::<Context>().expect("Missing auth context");

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        is_loading.set(true);

        error_msg.set(None);

        spawn_local(async move {
            let response = Request::post("/api/v1/users/login")
                .json(&LoginRequest::new(email.get(), password.get()))
                .expect("Failed to serialize login request")
                .send()
                .await;

            match response {
                Ok(res) => match res.status() {
                    201 => match res.json::<Response>().await {
                        Ok(body) => context.token.set(Some(body.access_token)),
                        Err(err) => {
                            error_msg.set(Some("Unexpected data format from server.".to_string()));
                            log!("Error decoding response: {}", err);
                        }
                    },
                    401 => {
                        error_msg.set(Some("Incorrect email or password.".to_string()));
                    }
                    _ => {
                        log!("Unexpected server response: {}", res.status());
                        error_msg.set(Some(
                            "A server error occurred. Please try again.".to_string(),
                        ));
                    }
                },
                Err(err) => {
                    log!("Network request failed: {:?}", err);
                    error_msg.set(Some("Failed to connect to the server.".to_string()));
                }
            }

            is_loading.set(false);
        })
    };

    div().class("auth-card").child((
        h2().child("Login"),
        form()
            .on(ev::submit, on_submit)
            .class("input-container")
            .child((
                input()
                    .attr("type", "email")
                    .attr("placeholder", "Email")
                    .attr("name", "email")
                    .prop("value", move || email.get())
                    .on(ev::input, move |ev| {
                        email.set(event_target_value(&ev));
                    }),
                input()
                    .attr("type", "password")
                    .attr("placeholder", "Password")
                    .attr("name", "password")
                    .prop("value", move || password.get())
                    .on(ev::input, move |ev| {
                        password.set(event_target_value(&ev));
                    }),
                move || {
                    error_msg
                        .get()
                        .map(|msg| span().class("error-text").child(msg))
                },
                button()
                    .attr("type", "button")
                    .class("link-btn")
                    .on(ev::click, move |_| set_mode.set(AuthMode::Register))
                    .child("Don't have an account? Register"),
                button()
                    .attr("type", "submit")
                    .class("auth-btn")
                    .attr("disabled", move || is_loading.get())
                    .child(move || {
                        if is_loading.get() {
                            Either::Left(ProgressBar())
                        } else {
                            Either::Right("Sign in")
                        }
                    }),
            )),
    ))
}

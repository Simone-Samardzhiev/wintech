use super::AuthMode;
use crate::widgets::ProgressBar;
use gloo_net::http::Request;
use leptos::html::span;
use leptos::{
    either::Either,
    ev,
    ev::SubmitEvent,
    html::{button, div, form, h2, input, p},
    leptos_dom::log,
    prelude::*,
    task::spawn_local,
};
use serde::Serialize;

/// Struct representing the JSON request for registering.
#[derive(Debug, Serialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

impl RegisterRequest {
    const MIN_USERNAME_LENGTH: usize = 8;
    const MAX_USERNAME_LENGTH: usize = 128;

    fn parse_username(mut username: String) -> Result<String, String> {
        username = username.trim().to_string();
        let count = username.chars().count();

        if count < Self::MIN_USERNAME_LENGTH {
            Err(format!(
                "Username length must be more than {} characters long.",
                Self::MIN_USERNAME_LENGTH
            ))
        } else if count > Self::MAX_USERNAME_LENGTH {
            Err(format!(
                "Username length must be less than {} characters long.",
                Self::MAX_USERNAME_LENGTH
            ))
        } else {
            Ok(username)
        }
    }

    fn parse_email(mut email: String) -> Result<String, String> {
        email = email.trim().to_string();

        if email_address::EmailAddress::is_valid(&email) {
            Ok(email)
        } else {
            Err("Invalid email address.".to_string())
        }
    }

    const MIN_PASSWORD_LENGTH: usize = 8;
    const MAX_PASSWORD_LENGTH: usize = 24;

    fn parse_password(mut password: String) -> Result<String, String> {
        password = password.trim().to_string();
        let count = password.chars().count();

        if count < Self::MIN_PASSWORD_LENGTH {
            return Err(format!(
                "Password length must be more than {} characters long.",
                Self::MIN_PASSWORD_LENGTH
            ));
        } else if count > Self::MAX_PASSWORD_LENGTH {
            return Err(format!(
                "Password length must be less than {} characters long.",
                Self::MAX_PASSWORD_LENGTH
            ));
        }

        let mut has_upper = false;
        let mut has_lower = false;
        let mut has_special = false;
        let mut has_digit = false;
        for c in password.chars() {
            if c.is_uppercase() {
                has_upper = true;
            }
            if c.is_lowercase() {
                has_lower = true;
            }
            if c.is_ascii_punctuation() {
                has_special = true;
            }
            if c.is_ascii_digit() {
                has_digit = true;
            }
        }

        if !has_upper {
            return Err("Password must have at least one uppercase character.".to_string());
        }
        if !has_lower {
            return Err("Password must have at least one lowercase character.".to_string());
        }
        if !has_special {
            return Err("Password must have at least one special character.".to_string());
        }
        if !has_digit {
            return Err("Password must have at least one digit.".to_string());
        }

        Ok(password)
    }

    pub fn parse(
        mut username: String,
        mut email: String,
        mut password: String,
    ) -> Result<Self, String> {
        username = Self::parse_username(username.clone())?;
        email = Self::parse_email(email.clone())?;
        password = Self::parse_password(password.clone())?;

        Ok(Self {
            username,
            email,
            password,
        })
    }
}

/// Component used to register.
#[component]
pub fn Register(set_mode: WriteSignal<AuthMode>) -> impl IntoView {
    let username = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());

    let is_loading = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);
    let is_success = RwSignal::new(false);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        is_loading.set(true);

        spawn_local(async move {
            let payload = match RegisterRequest::parse(username.get(), email.get(), password.get())
            {
                Ok(payload) => payload,
                Err(err) => {
                    error_msg.set(Some(err));
                    is_loading.set(false);
                    return;
                }
            };

            let response = Request::post("/api/v1/users/register")
                .json(&payload)
                .expect("Failed to serialize register request")
                .send()
                .await;

            match response {
                Ok(res) => match res.status() {
                    201 => {
                        is_success.set(true);
                        username.set(String::new());
                        email.set(String::new());
                        password.set(String::new());
                    }
                    409 => {
                        error_msg.set(Some("Email is already in use.".to_string()));
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
        h2().child("Register"),
        form().on(ev::submit, on_submit).child(move || {
            if is_success.get() {
                Either::Left(
                    div().class("success-container").child((
                        p().class("success-text")
                            .child("Account created successfully!"),
                        button()
                            .on(ev::click, move |_| set_mode.set(AuthMode::Login))
                            .class("auth-btn")
                            .child("Go to Login"),
                    )),
                )
            } else {
                Either::Right(
                    form()
                        .on(ev::submit, on_submit)
                        .class("input-container")
                        .child((
                            input()
                                .attr("type", "text")
                                .attr("placeholder", "Username")
                                .attr("name", "username")
                                .prop("value", move || username.get())
                                .on(ev::input, move |ev| {
                                    username.set(event_target_value(&ev));
                                }),
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
                                .on(ev::click, move |_| set_mode.set(AuthMode::Login))
                                .child("Don't have an account? Register"),
                            button()
                                .attr("type", "submit")
                                .class("auth-btn")
                                .attr("disabled", move || is_loading.get())
                                .child(move || {
                                    if is_loading.get() {
                                        Either::Left(ProgressBar())
                                    } else {
                                        Either::Right("Create Account")
                                    }
                                }),
                        )),
                )
            }
        }),
    ))
}

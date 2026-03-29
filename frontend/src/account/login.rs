use super::AuthMode;
use crate::{AuthContext, widgets::ProgressBar};
use gloo_net::http::Request;
use leptos::{
    context::use_context, ev::SubmitEvent, leptos_dom::log, prelude::*, task::spawn_local,
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
    let context = use_context::<AuthContext>().expect("Missing auth context");

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
                    201 => {
                        context.is_logged_in.set(true);
                    }
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

    view! {
        <div class="auth-card">
            <h2>"Login"</h2>
            <form on:submit=on_submit class="input-container">
                <input type="email" placeholder="Email" name="email" bind:value=email/>
                <input type="password" placeholder="Password" name="password" bind:value=password/>

                {move || error_msg.get().map(|msg| view! { <span class="error-text">{msg}</span> })}

                <button type="button" class="link-btn" on:click=move |_| set_mode.set(AuthMode::Register)>
                    "Don't have an account? Register"
                </button>
                <button
                    type="submit"
                    class="auth-btn"
                    disabled=is_loading
                >
                    <Show
                        when=move || !is_loading.get()
                        fallback=move || view! { <ProgressBar /> }
                    >
                        "Sign In"
                    </Show>
                </button>
            </form>
        </div>
    }
}

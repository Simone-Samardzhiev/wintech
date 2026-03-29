mod login;

use crate::AuthContext;
use leptos::prelude::*;

/// Enum used to swap between [`Register`] component and [`login::Login`] component.
#[derive(Clone, Copy, PartialEq, Default)]
enum AuthMode {
    #[default]
    Login,
    Register,
}

/// Component displaying either [`Register`] or [`login::Login`] component.
#[component]
fn Auth() -> impl IntoView {
    let auth_mode = RwSignal::new(AuthMode::Login);

    view! {
        <div class="auth-page">
            <Show
                when=move || auth_mode.get() == AuthMode::Login
                fallback=move || view! { <Register set_mode=auth_mode.write_only() /> }
            >
                <login::Login set_mode=auth_mode.write_only() />
            </Show>
        </div>
    }
}

/// Component used to register.
#[component]
fn Register(set_mode: WriteSignal<AuthMode>) -> impl IntoView {
    let username = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());

    view! {
        <div class="auth-card">
            <h2>"Register"</h2>
            <form class="input-container">
                <input type="text" placeholder="Username" name="username" bind:value=username/>
                <input type="text" placeholder="Email" name="email" bind:value=email/>
                <input type="password" placeholder="Password" name="password" bind:value=password/>
                <button class="link-btn" on:click=move |_| set_mode.set(AuthMode::Login)>
                    "Already have an account? Login"
                </button>
                <button type="submit" class="auth-btn">"Create Account"</button>
            </form>
        </div>
    }
}

/// Component displaying orders and info about already installed smart windows.
#[component]
fn Dashboard() -> impl IntoView {
    view! {
        <h1>"Dashboard"</h1>
    }
}

/// Component that displays [`Auth`] if the user is not logged in
/// and [`Dashboard`] otherwise.
#[component]
pub fn Account() -> impl IntoView {
    let context = use_context::<AuthContext>().expect("Missing auth context");

    view! {
        <Show when=move || context.is_logged_in.get() fallback=move || view! {<Auth/>}>
            <Dashboard/>
        </Show>
    }
}

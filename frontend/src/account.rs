mod login;
mod register;

use crate::auth;
use leptos::prelude::*;
use login::Login;
use register::Register;

/// Enum used to swap between [`Register`] component and [`Login`] component.
#[derive(Clone, Copy, PartialEq, Default)]
enum AuthMode {
    #[default]
    Login,
    Register,
}

/// Component displaying either [`Register`] or [`Login`] component.
#[component]
fn Auth() -> impl IntoView {
    let auth_mode = RwSignal::new(AuthMode::Login);

    view! {
        <div class="auth-page">
            <Show
                when=move || auth_mode.get() == AuthMode::Login
                fallback=move || view! { <Register set_mode=auth_mode.write_only() /> }
            >
                <Login set_mode=auth_mode.write_only() />
            </Show>
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
    let context = use_context::<auth::Context>().expect("Missing auth context");

    view! {
        <Show when=move || context.is_expired() fallback=move || view! {<Auth/>}>
            <Dashboard/>
        </Show>
    }
}

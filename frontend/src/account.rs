mod dashboard;
mod login;
mod register;

use crate::account::dashboard::Dashboard;
use crate::auth;
use leptos::{either::Either, html::div, prelude::*};
use login::{Login, LoginProps};
use register::{Register, RegisterProps};

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
    div()
        .class("auth-page")
        .child(move || match auth_mode.get() {
            AuthMode::Login => Either::Left(Login(LoginProps {
                set_mode: auth_mode.write_only(),
            })),
            AuthMode::Register => Either::Right(Register(RegisterProps {
                set_mode: auth_mode.write_only(),
            })),
        })
}

/// Component that displays [`Auth`] if the user is not logged in
/// and [`Dashboard`] otherwise.
#[component]
pub fn Account() -> impl IntoView {
    let context = use_context::<auth::Context>().expect("Missing auth context");

    move || match context.token.get() {
        Some(_) => Either::Left(Dashboard()),
        None => Either::Right(Auth()),
    }
}

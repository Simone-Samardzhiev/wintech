use leptos::{html::{a, div, span}, prelude::*};

#[component]
pub fn ProgressBar() -> impl IntoView {
    div().class("progress-bar")
}

#[component]
pub fn ToLoginButton(cause: String) -> impl IntoView {
    div()
        .class("center-wrapper")
        .child(a().attr("href", "/account").class("login-link").child((
            span().class("login-label").child("Log In to Proceed"),
            span().class("login-cause").child(cause),
        )))
}

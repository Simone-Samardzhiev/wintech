mod account;
mod auth;
mod home;
mod widgets;

use leptos::{
    either::Either,
    html::{div, img, main as html_main, nav},
    leptos_dom::error,
    prelude::*,
    task::spawn_local,
};
use leptos_router::{
    components::{A, AProps, Route, RouteProps, Router, RouterProps, Routes, RoutesProps},
    path,
};

use crate::widgets::ProgressBar;
use auth::Error as AuthError;

/// Custom view displayed when a page is not found.
#[component]
pub fn NotFound() -> impl IntoView {
    div().attr("class", "not-found-container").child(
        img()
            .class("not-found-img")
            .src("/assets/not_found.png")
            .alt("Oops! It looks like this page is broken."),
    )
}

/// The app entry point.
#[component]
fn App() -> impl IntoView {
    let context = auth::Context::default();
    let session_check = RwSignal::new(false);

    spawn_local(async move {
        match auth::refresh_session().await {
            Ok(res) => context.token.set(Some(res.access_token)),
            Err(AuthError::ExpiredSession) => {}
            Err(AuthError::UnexpectedResponse(err)) => {
                error!("Error decoding response: {:?}", err);
            }
            Err(AuthError::UnexpectedStatusCode(code)) => {
                error!("Unexpected response status code: {}", code);
            }
            Err(AuthError::Network(err)) => {
                error!("Network request failed: {:?}", err);
            }
        }
        session_check.set(true);
    });

    provide_context(context);

    move || {
        if session_check.get() {
            Either::Left(Router(
                RouterProps::builder()
                    .children(ToChildren::to_children(|| {
                        (
                            nav().child((
                                A(AProps::builder()
                                    .href("/")
                                    .children(ToChildren::to_children(|| "Home"))
                                    .build()),
                                A(AProps::builder()
                                    .href("/account")
                                    .children(ToChildren::to_children(|| "Account"))
                                    .build()),
                            )),
                            html_main().child(Routes(
                                RoutesProps::builder()
                                    .fallback(|| NotFound())
                                    .children(ToChildren::to_children(|| {
                                        (
                                            Route(
                                                RouteProps::builder()
                                                    .path(path!("/"))
                                                    .view(home::Home)
                                                    .build(),
                                            ),
                                            Route(
                                                RouteProps::builder()
                                                    .path(path!("/account"))
                                                    .view(account::Account)
                                                    .build(),
                                            ),
                                        )
                                    }))
                                    .build(),
                            )),
                        )
                    }))
                    .build(),
            ))
        } else {
            Either::Right(div().class("progress-container").child(ProgressBar()))
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}

mod account;
mod auth;
mod home;
mod widgets;

use leptos::html::{div, img, main as html_main, nav};
use leptos::prelude::*;
use leptos_router::{
    components::{A, AProps, Route, RouteProps, Router, RouterProps, Routes, RoutesProps},
    path,
};

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
    provide_context(context);

    Router(
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
                            .fallback(|| NotFound().into_view())
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
    )
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}

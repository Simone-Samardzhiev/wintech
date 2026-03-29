mod account;
mod home;
mod widgets;

use leptos::prelude::*;
use leptos_router::{
    components::{A, Route, Router, Routes},
    path,
};

/// Custom view displayed when a page is not found.
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        // Wrap the image in a container
        <div class="not-found-container">
            <img class="not-found-img" src="/assets/not_found.png" alt="Oops! It looks like this page is broken." />
        </div>
    }
}

#[derive(Copy, Clone)]
struct AuthContext {
    is_logged_in: RwSignal<bool>,
}

/// The app entry point.
#[component]
fn App() -> impl IntoView {
    provide_context(AuthContext {
        is_logged_in: RwSignal::new(false),
    });

    view! {
        <Router>
            <nav>
                <A href="/">"Home"</A>
                <A href="/account">"Account"</A>
            </nav>
            <main>
                <Routes fallback=|| view! { <NotFound/> }>
                    <Route path=path!("/") view=home::Home/>
                    <Route path=path!("/account") view=account::Account/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}

mod home;

use leptos::prelude::*;
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::path;

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

/// The app entry point.
#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"Home"</A>
                <A href="/order">"Order"</A>
            </nav>
            <main>
                <Routes fallback=|| view! { <NotFound/> }>
                    <Route path=path!("/") view=home::Home/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(App)
}

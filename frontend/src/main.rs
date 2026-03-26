mod home;
mod order;

use leptos::prelude::*;
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::path;

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"Home"</A>
                <A href="/order">"Order"</A>
            </nav>
            <main>
                <Routes fallback=|| view! { "Not found" }>
                    <Route path=path!("/") view=home::Home/>
                    <Route path=path!("/order") view=order::Order />
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(App)
}

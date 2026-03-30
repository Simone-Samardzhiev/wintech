mod account;
mod auth;
mod home;
mod widgets;

use auth::{Error as AuthError, State as AuthState};
use codee::string::FromToStringCodec;
use leptos::logging::log;
use leptos::{prelude::*, task::spawn_local};
use leptos_router::{
    components::{A, Route, Router, Routes},
    path,
};
use leptos_use::storage::use_local_storage;

/// Custom view displayed when a page is not found.
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="not-found-container">
            <img class="not-found-img" src="/assets/not_found.png" alt="Oops! It looks like this page is broken." />
        </div>
    }
}

/// The app entry point.
#[component]
fn App() -> impl IntoView {
    let context = auth::Context::default();
    let (get_previously_logged, _, _) =
        use_local_storage::<bool, FromToStringCodec>("previously_logged");

    if get_previously_logged.get() {
        spawn_local(async move {
            match auth::refresh_session().await {
                Ok(response) => context.state.set(AuthState::Logged(response.access_token)),
                Err(err) => match err {
                    AuthError::SessionExpired => {
                        context.state.set(AuthState::Error("".to_string()))
                    }
                    AuthError::InvalidResponse => {
                        log!("Unexpected server response");
                    }
                    AuthError::Network(_) => {
                        log!("Network error");
                    }
                },
            }
        });
    }

    provide_context(context);

    view! {
        <Router>
            <auth::AuthAlert/>
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

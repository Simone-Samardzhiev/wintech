use leptos::ev;
use leptos::html::{button, div, h1, p};
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let counter = RwSignal::new(0);

    div().child((
        h1().child("Hello from Rust!"),
        button()
            .on(ev::click, move |_| *counter.write() += 1)
            .child("Increment"),
        button()
            .on(ev::click, move |_| *counter.write() -= 1)
            .child("Decrement"),
        p().child(("Counter: ", counter)),
    ))
}

fn main() {
    mount_to_body(App)
}

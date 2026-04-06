mod place_order;

use leptos::either::Either;
use leptos::html::div;
use leptos::prelude::*;

use crate::auth;
use crate::orders::place_order::PlaceOrderProps;
use crate::widgets;

#[component]
pub fn Orders() -> impl IntoView {
    let context = use_context::<auth::Context>().expect("Missing auth context");

    div().child(move || match context.token.get() {
        Some(token) => Either::Left(place_order::PlaceOrder(PlaceOrderProps { token })),
        None => Either::Right(widgets::ToLoginButton(widgets::ToLoginButtonProps {
            cause: "Please sign in to place or manage your orders.".into(),
        })),
    })
}

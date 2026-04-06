use crate::auth;
use crate::widgets::ProgressBar;
use gloo_net::http::{Method, RequestBuilder};
use leptos::task::spawn_local;
use leptos::{
    either::Either,
    ev,
    html::{button, div, form, h2, input, label, p, span},
    logging::{error},
    prelude::*,
};
use serde::Serialize;

#[derive(Serialize)]
struct OrderRequest {
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    pub city: String,
    pub neighborhood: Option<String>,
    pub street: String,
    #[serde(rename = "floorLevel")]
    pub floor_level: Option<i16>,
}

impl OrderRequest {
    const MIN_LENGTH: usize = 3;
    const MAX_LENGTH: usize = 100;
    pub fn parse(
        mut postal_code: String,
        mut city: String,
        mut neighborhood: Option<String>,
        mut street: String,
        floor_level: Option<i16>,
    ) -> Result<Self, String> {
        postal_code = postal_code.trim().to_string();
        city = city.trim().to_string();
        neighborhood = neighborhood.map(|n| n.trim().to_string());
        street = street.trim().to_string();

        let count = postal_code.chars().count();
        if count < Self::MIN_LENGTH || count > Self::MAX_LENGTH {
            return Err(format!(
                "Postal code must be between {}-{} characters.",
                Self::MIN_LENGTH,
                Self::MAX_LENGTH
            ));
        }

        let count = city.chars().count();
        if count < Self::MIN_LENGTH || count > Self::MAX_LENGTH {
            return Err(format!(
                "City must be between {}-{} characters.",
                Self::MIN_LENGTH,
                Self::MAX_LENGTH
            ));
        }

        let count = neighborhood.as_ref().map(|n| n.chars().count());
        if let Some(count) = count {
            if count < Self::MIN_LENGTH || count > Self::MAX_LENGTH {
                return Err(format!(
                    "Neighborhood must be between {}-{} character.",
                    Self::MIN_LENGTH,
                    Self::MAX_LENGTH
                ));
            }
        }

        let count = street.chars().count();
        if count < Self::MIN_LENGTH || count > Self::MAX_LENGTH {
            return Err(format!(
                "Street must be between {}-{} characters.",
                Self::MIN_LENGTH,
                Self::MAX_LENGTH
            ));
        }

        Ok(Self {
            postal_code,
            city,
            neighborhood,
            street,
            floor_level,
        })
    }
}

impl TryFrom<Order> for OrderRequest {
    type Error = String;
    fn try_from(order: Order) -> Result<Self, Self::Error> {
        Self::parse(
            order.postal_code,
            order.city,
            order.neighborhood,
            order.street,
            order.floor_level,
        )
    }
}

#[derive(Clone, Default, Debug)]
struct Order {
    pub postal_code: String,
    pub city: String,
    pub neighborhood: Option<String>,
    pub street: String,
    pub floor_level: Option<i16>,
}

#[component]
fn OrderCompleteWidget() -> impl IntoView {
    div().class("order-complete-widget").child((
        h2().class("order-complete-title")
            .child("Order Complete"),
        p().class("order-complete-message")
            .child("Your order has been successfully placed!"),
    ))
}

#[component]
pub fn PlaceOrder(token: String) -> impl IntoView {
    let context = use_context::<auth::Context>().expect("Missing auth context");
    let payload = RwSignal::new(Order::default());

    let is_loading = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);
    let is_complete = RwSignal::new(false);

    let stored_token = StoredValue::new(token);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        let current_token = stored_token.get_value();
        let ctx = context.clone();

        spawn_local(async move {
            is_loading.set(true);
            error_msg.set(None);

            let body = match OrderRequest::try_from(payload.get()) {
                Ok(body) => body,
                Err(e) => {
                    error_msg.set(Some(e));
                    is_loading.set(false);
                    return;
                }
            };

            let response = auth::authenticate_request(
                &current_token,
                move || RequestBuilder::new("api/v1/orders").method(Method::POST),
                Some(body),
            )
                .await;

            match response {
                Ok(res) => {
                    if let Some(access_token) = res.access_token {
                        ctx.token.set(Some(access_token));
                    }

                    match res.http_response.status() {
                        201 => {
                            is_complete.set(true);
                        }
                        _ => {
                            error!(
                                "Unexpected response status code: {}",
                                res.http_response.status()
                            );
                            error_msg.set(Some(
                                "A server error occurred. Please try again.".to_string(),
                            ));
                        }
                    }
                }
                Err(error) => {
                    error!("Error placing order: {}", error);
                    ctx.token.set(None);
                }
            }

            is_loading.set(false);
        })
    };

    move || {
        if is_complete.get() {
            Either::Right(OrderCompleteWidget())
        } else {
            Either::Left(
                form().class("order-form").on(ev::submit, on_submit.clone()).child((
                    h2().class("form-title").child("Shipping Details"),
                    move || {
                        error_msg.get().map(|msg| {
                            div().class("error-msg")
                                .child(msg)
                        })
                    },
                    div().class("input-group").child((
                        label().attr("for", "postalCode").child("Postal Code *"),
                        input()
                            .id("postalCode")
                            .attr("type", "text")
                            .attr("required", true)
                            .attr("minlength", "3")
                            .attr("maxlength", "12")
                            .prop("value", move || payload.read().postal_code.clone())
                            .on(ev::input, move |ev| {
                                payload.update(|p| p.postal_code = event_target_value(&ev));
                            }),
                    )),
                    div().class("input-group").child((
                        label().attr("for", "city").child("City *"),
                        input()
                            .id("city")
                            .attr("type", "text")
                            .attr("required", true)
                            .attr("minlength", "3")
                            .attr("maxlength", "100")
                            .prop("value", move || payload.read().city.clone())
                            .on(ev::input, move |ev| {
                                payload.update(|p| p.city = event_target_value(&ev));
                            }),
                    )),
                    div().class("input-group").child((
                        label().attr("for", "neighborhood").child((
                            "Neighborhood ",
                            span()
                                .style("color: #6b7280")
                                .style("font-weight: normal")
                                .child("(Optional)"),
                        )),
                        input()
                            .id("neighborhood")
                            .attr("type", "text")
                            .attr("minlength", "3")
                            .attr("maxlength", "100")
                            .prop("value", move || {
                                payload.read().neighborhood.clone().unwrap_or_default()
                            })
                            .on(ev::input, move |ev| {
                                let val = event_target_value(&ev);
                                payload.update(|p| {
                                    p.neighborhood = if val.trim().is_empty() {
                                        None
                                    } else {
                                        Some(val)
                                    }
                                });
                            }),
                    )),
                    div().class("input-group").child((
                        label().attr("for", "street").child("Street *"),
                        input()
                            .id("street")
                            .attr("type", "text")
                            .attr("required", true)
                            .attr("minlength", "3")
                            .attr("maxlength", "100")
                            .prop("value", move || payload.read().street.clone())
                            .on(ev::input, move |ev| {
                                payload.update(|p| p.street = event_target_value(&ev));
                            }),
                    )),
                    div().class("input-group").child((
                        label().attr("for", "floorLevel").child((
                            "Floor Level ",
                            span()
                                .style("color: #6b7280")
                                .style("font-weight: normal")
                                .child("(Optional)"),
                        )),
                        input()
                            .id("floorLevel")
                            .attr("type", "number")
                            .attr("min", "0")
                            .attr("max", "32767")
                            .prop("value", move || {
                                payload
                                    .read()
                                    .floor_level
                                    .map(|v| v.to_string())
                                    .unwrap_or_default()
                            })
                            .on(ev::input, move |ev| {
                                let val = event_target_value(&ev);
                                payload.update(|p| p.floor_level = val.parse::<i16>().ok());
                            }),
                    )),
                    button()
                        .class("submit-btn")
                        .attr("type", "submit")
                        .attr("disabled", move || is_loading.get())
                        .child(move || {
                            if is_loading.get() {
                                Either::Right(ProgressBar())
                            } else {
                                Either::Left("Order")
                            }
                        }),
                ))
            )
        }
    }
}
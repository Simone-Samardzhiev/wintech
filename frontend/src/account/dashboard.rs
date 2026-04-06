use crate::auth;
use crate::widgets::ProgressBar;
use gloo_net::http::RequestBuilder;
use leptos::{
    either::Either,
    html::{div, h1, p, span},
    leptos_dom::error,
    prelude::*,
    task::spawn_local,
};
use serde::Deserialize;
use time::Time;
use uuid::Uuid;

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Window {
    id: Uuid,
    #[serde(rename = "preferredTemperature")]
    preferred_temperature: i16,
    #[serde(rename = "preferredWakeUpTime")]
    preferred_wakeup_time: Time,
    #[serde(rename = "preferredBedTime")]
    preferred_bedtime: Time,
}

#[component]
pub fn Windows(windows: RwSignal<Vec<Window>>) -> impl IntoView {
    div().class("dashboard-container").child((
        h1().class("dashboard-title").child("My Windows"),
        div().class("windows-grid").child(For(ForProps::builder()
            .each(move || windows.get())
            .key(|window| window.id)
            .children(|window| {
                div().class("window-card").child((
                    div().class("card-header").child(format!(
                        "Window: {}",
                        window.id.to_string()[..8].to_string()
                    )),
                    div().class("card-content").child((
                        p().child(format!("Temp: {}°C", window.preferred_temperature)),
                        p().child(format!(
                            "Wake: {:02}:{:02}",
                            window.preferred_wakeup_time.hour(),
                            window.preferred_wakeup_time.minute()
                        )),
                        p().child(format!(
                            "Bed: {:02}:{:02}",
                            window.preferred_bedtime.hour(),
                            window.preferred_bedtime.minute()
                        )),
                    )),
                ))
            })
            .build())),
    ))
}

/// Component displaying orders and info about already installed smart windows.
#[component]
pub fn Dashboard(token: String) -> impl IntoView {
    let context = use_context::<auth::Context>().expect("Missing auth context");
    let is_loaded = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);
    let windows = RwSignal::new(Vec::<Window>::new());

    spawn_local(async move {
        let response = auth::authenticate_request(
            &token,
            move || RequestBuilder::new("api/v1/windows"),
            None::<()>,
        )
        .await;

        match response {
            Ok(res) => {
                match res.access_token {
                    Some(access_token) => context.token.set(Some(access_token)),
                    None => {}
                }
                match res.http_response.status() {
                    200 => match res.http_response.json::<Vec<Window>>().await {
                        Ok(body) => windows.set(body),
                        Err(err) => {
                            error_msg.set(Some("Unexpected response from server.".to_string()));
                            error!("Error decoding response: {}", err);
                        }
                    },
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
                error!("Error refreshing session: {}", error);
                context.token.set(None);
            }
        };

        is_loaded.set(true);
    });

    move || {
        if is_loaded.get() {
            Either::Left(div().child((
                move || {
                    error_msg
                        .get()
                        .map(|msg| span().class("error-text").child(msg))
                },
                Windows(WindowsProps { windows }),
            )))
        } else {
            Either::Right(div().class("progress-container").child(ProgressBar()))
        }
    }
}

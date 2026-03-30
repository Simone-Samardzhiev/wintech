use leptos::prelude::*;
use serde::Serialize;
use time::Time;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct WindowResponse {
    id: Uuid,
    #[serde(rename = "preferredTemperature")]
    preferred_temperature: i16,
    #[serde(rename = "preferredWakeUpTime")]
    preferred_wakeup_time: Time,
    #[serde(rename = "preferredBedTime")]
    preferred_bedtime: Time,
}

/// Component displaying orders and info about already installed smart windows.
#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <h1>"Dashboard"</h1>
    }
}

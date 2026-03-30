use leptos::html::{div, h1, h2, img, p};
use leptos::prelude::*;

/// Struct representing a feature of the smart window.
pub struct Feature {
    title: &'static str,
    description: &'static str,
    color: &'static str,
    image_url: &'static str,
}

impl Feature {
    fn new(
        title: &'static str,
        description: &'static str,
        color: &'static str,
        image_url: &'static str,
    ) -> Self {
        Self {
            title,
            description,
            color,
            image_url,
        }
    }
}

/// View that display a feature of the smart window.
#[component]
pub fn FeatureCard(feature: Feature) -> impl IntoView {
    div()
        .class("feature-card")
        .style(format!("--card-color: {}", feature.color))
        .child((
            div()
                .class("feature-card-content")
                .child((h2().child(feature.title), p().child(feature.description))),
            img().src(feature.image_url).alt(feature.title),
        ))
}

/// The start/home page.
#[component]
pub fn Home() -> impl IntoView {
    let features = vec![
        Feature::new(
            "Natural Routine",
            "Sync the window with your sleep schedule. \
            It will automatically secure the room and close the blinds at night, \
            then gently open them shortly before your alarm to wake you up with natural sunlight.",
            "#FFEA6E",
            "/assets/natural_routine.png",
        ),
        Feature::new(
            "Smart Climate",
            "Set your preferred temperature and the window will intelligently manage the blinds and airflow. \
            By leveraging the outside air, it keeps you comfortable \
            while drastically reducing your air conditioning costs.",
            "#00C712",
            "/assets/smart_climate.png",
        ),
        Feature::new(
            "Rain Shield",
            "Never worry about an open window again. \
            If you're out and the weather turns, \
            the system detects rain instantly and closes itself, \
            keeping your home dry and tidy.",
            "#004CC7",
            "/assets/rain_shield.png",
        ),
    ];

    (
        h1().class("title").child("Smart window systems"),
        h1().class("subtitle")
            .child("The perfect addition for your smart home"),
        div().class("features").child(
            features
                .into_iter()
                .map(|f| FeatureCard(FeatureCardProps { feature: f }))
                .collect_view(),
        ),
    )
}

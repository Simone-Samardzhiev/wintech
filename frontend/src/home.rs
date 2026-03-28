use leptos::prelude::*;

/// View that display a feature of the smart window.
#[component]
pub fn FeatureCard(
    title: &'static str,
    description: &'static str,
    color: &'static str,
    image_url: &'static str,
) -> impl IntoView {
    view! {
    <div class="feature-card" style=format!("--card-color: {color}")>
        <div class="card-content">
            <h2>{title}</h2>
            <p>{description}</p>
        </div>
        <img src=image_url alt=title />
    </div>
    }
}

/// The start/home page.
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <h1 class="title">"Smart window systems"</h1>
        <h1 class="subtitle">"The perfect addition for your smart home"</h1>
        <div class="features">
            <FeatureCard
            title="Natural Routine"
            description="Sync the window with your sleep schedule. \
            It will automatically secure the room and close the blinds at night, \
            then gently open them shortly before your alarm to wake you up with natural sunlight."
            color="#FFEA6E"
            image_url="/assets/natural_routine.png"/>

            <FeatureCard
            title="Smart Climate"
            description="Set your preferred temperature and the window will intelligently manage the blinds and airflow. \
            By leveraging the outside air, it keeps you comfortable \
            while drastically reducing your air conditioning costs."
            color="#00C712"
            image_url="/assets/smart_climate.png"/>

            <FeatureCard
            title="Rain Shield"
            description="Never worry about an open window again. \
            If you're out and the weather turns, \
            the system detects rain instantly and closes itself, \
            keeping your home dry and tidy"
            color="#004CC7"
            image_url="/assets/rain_shield.png"/>
        </div>


    }
}

use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

mod config;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let cnf = config::Config::new().unwrap();
    let dist_path = "../frontend/dist";

    let serve_dir = ServeDir::new(dist_path)
        .append_index_html_on_directories(true)
        .fallback(ServeFile::new(format!("{}/index.html", dist_path)));

    let router = Router::new()
        .nest_service("/assets", ServeDir::new(format!("{}/assets", dist_path)))
        .fallback_service(serve_dir);

    let listener = tokio::net::TcpListener::bind(&cnf.address).await.unwrap();
    println!("Serving frontend from {} on {}", dist_path, cnf.address);
    axum::serve(listener, router).await.unwrap();
}

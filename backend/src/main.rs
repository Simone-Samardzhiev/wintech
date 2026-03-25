use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

mod config;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let cnf = config::Config::new().unwrap();
    let dist_path = "../frontend/dist";

    let serve_dir =
        ServeDir::new(dist_path).fallback(ServeFile::new(format!("{}/index.html", dist_path)));

    let router = Router::new().fallback_service(serve_dir);

    let listener = tokio::net::TcpListener::bind(&cnf.address).await.unwrap();
    println!("Serving frontend from {} on {}", dist_path, cnf.address);
    axum::serve(listener, router).await.unwrap();
}

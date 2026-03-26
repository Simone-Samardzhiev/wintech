use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

mod config;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let cnf = config::Config::new().unwrap();

    let router = Router::new().fallback_service(ServeDir::new(&cnf.frontend_path).not_found_service(
        ServeFile::new(std::path::Path::new(&cnf.frontend_path).join("index.html")),
    ));

    let listener = tokio::net::TcpListener::bind(&cnf.address).await.unwrap();
    println!("Serving frontend from {} on {}", cnf.frontend_path, cnf.address);
    axum::serve(listener, router).await.unwrap();
}

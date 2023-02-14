use axum::{
    http::{StatusCode, Uri},
    response::Html,
    routing::get,
    Router,
};
use dotenv::dotenv;
use std::net::SocketAddr;

mod handlers;
pub mod indices;

async fn root() -> Html<&'static str> {
    Html(
        "<!DOCTYPE html>
        <html>
        <head>
            <meta name='color-scheme' content='dark'></meta>
            <title>Giga Stonks API</title>
            <style>
            body {
                font-family: Georgia, sans-serif;
            }
            </style>
        </head>
        <body>
            <h1>Giga Stonks API</h1>
        </body>
        </html>",
    )
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    // Routes setup
    let api_routes_v1 = Router::new()
        .route("/market-news", get(handlers::finnhub::get_market_news))
        .route(
            "/market-status",
            get(handlers::alphavantage::get_market_status),
        );

    // App setup
    let app = Router::new()
        .route("/", get(root))
        .nest("/api/v1", api_routes_v1)
        .fallback(fallback);

    // Server setup
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

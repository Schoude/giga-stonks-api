use axum::{
    http::{StatusCode, Uri},
    response::Html,
    routing::get,
    Router,
};
use dotenv::dotenv;
use sync_wrapper::SyncWrapper;

pub mod alphavantage_api;
mod finnhub_api;
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

#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    // Routes setup
    let api_routes_v1 = Router::new()
        .route("/market-news", get(handlers::finnhub::get_market_news))
        .route(
            "/market-status",
            get(handlers::alphavantage::get_market_status),
        )
        .route(
            "/quotes/:index",
            get(handlers::finnhub::get_quotes_for_index),
        );

    // App setup
    let app = Router::new()
        .route("/", get(root))
        .nest("/api/v1", api_routes_v1)
        .fallback(fallback);

    let sync_wrapper = SyncWrapper::new(app);

    Ok(sync_wrapper)

    // Server setup
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // tracing::debug!("listening on {}", addr);
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use std::net::SocketAddr;
use tracing;
use tracing_subscriber;

use dotenv::dotenv;
use finnhub::{Endpoint, FinnhubAPI};

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

// Add this
// rename to giga-stonks-api
// https://crates.io/crates/axum
#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));
    // `POST /users` goes to `create_user`
    // .route("/users", post(create_user));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    // dotenv().ok();
    // let finnhub_api_token: String =
    //     std::env::var("FINNHUB_API_TOKEN").expect("FINNHUB_API_TOKEN must be set.");

    // let mut fh_api = FinnhubAPI::new(&finnhub_api_token);
    // fh_api.endpoint(Endpoint::MarketNews);

    // let articles = fh_api
    //     .fetch_market_news()
    //     .await
    //     .expect("The market news to be fetched");
}

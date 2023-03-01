use axum::{
    http::{Method, StatusCode, Uri},
    response::Html,
    routing::get,
    Router,
};
use shuttle_secrets::SecretStore;
use std::sync::Arc;
use sync_wrapper::SyncWrapper;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

pub mod alphavantage_api;
mod finnhub_api;
mod handlers;
pub mod indices;

pub struct AppState {
    api_token_finnhub: String,
    api_token_alphavantage: String,
}

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
async fn axum(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleAxum {
    let api_token_finnhub = if let Some(secret) = secret_store.get("FINNHUB_API_TOKEN") {
        secret
    } else {
        "Finnhub Api Token Not Set".to_string()
    };
    let api_token_alphavantage = if let Some(secret) = secret_store.get("ALPHA_VANTAGE_API_TOKEN") {
        secret
    } else {
        "Alpha Vantage Api Token Not Set".to_string()
    };

    let app_state = Arc::new(AppState {
        api_token_finnhub,
        api_token_alphavantage,
    });

    // CORS setup via tower service in middleware layer
    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET])
        .allow_origin(Any)
        .allow_credentials(false);

    let service = ServiceBuilder::new().layer(cors);

    // Routes setup
    let api_routes_v1 = Router::new()
        .layer(service)
        .route("/market-news", get(handlers::finnhub::get_market_news))
        .route(
            // /api/v1/company-news?symbol=XXXX&time_from=yyyy-mm-dd&time_to=yyyy-mm-dd
            "/company-news",
            get(handlers::finnhub::get_company_news),
        )
        .route(
            "/market-status",
            get(handlers::alphavantage::get_market_status),
        )
        .route(
            // /api/v1/news-sentiment?time_from=yyyymmdd
            "/news-sentiment",
            get(handlers::alphavantage::get_news_sentiment),
        )
        .route(
            // /api/v1/news-sentiment-ticker?ticker=XXXX?time_from=yyyymmdd
            "/news-sentiment-ticker",
            get(handlers::alphavantage::get_news_sentiment_ticker),
        )
        .route(
            // /api/v1/social-sentiment?symbol=XXXX&time_from=yyyy-mm-dd
            "/social-sentiment",
            get(handlers::finnhub::get_social_sentiment),
        )
        .route(
            "/quotes/:index",
            get(handlers::finnhub::get_quotes_for_index),
        )
        .route(
            "/company-profile/:symbol",
            get(handlers::finnhub::get_company_profile),
        );

    // App setup
    let app = Router::new()
        .route("/", get(root))
        .nest("/api/v1", api_routes_v1)
        .fallback(fallback)
        .with_state(app_state);

    let sync_wrapper = SyncWrapper::new(app);

    Ok(sync_wrapper)
}

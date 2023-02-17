use crate::finnhub_api::lib::{Endpoint, FinnhubAPI};
use crate::finnhub_api::market_news::ArticleMarketNews;
use crate::finnhub_api::symbol_quote::SymbolQuoteFrontend;
use crate::indices::{DOW_JONES, NASDAQ};
use axum::extract::Path;
use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

fn setup_finnhub_api(endpoint: Endpoint) -> FinnhubAPI {
    let finnhub_api_token: String =
        std::env::var("FINNHUB_API_TOKEN").expect("FINNHUB_API_TOKEN must be set.");

    let mut finnhub_api = FinnhubAPI::new(&finnhub_api_token);
    finnhub_api.endpoint(endpoint);
    finnhub_api
}

pub async fn get_market_news() -> (StatusCode, Json<Vec<ArticleMarketNews>>) {
    let fh_api = setup_finnhub_api(Endpoint::MarketNews);
    let articles = fh_api
        .fetch_market_news()
        .await
        .expect("The market news to be fetched");
    (StatusCode::OK, Json(articles))
}

pub async fn get_quotes_for_index(Path(index): Path<String>) -> (StatusCode, Json<Value>) {
    let fh_api = setup_finnhub_api(Endpoint::Quote);

    let market = match index.as_str() {
        "djia" => DOW_JONES,
        "nasdaq" => NASDAQ,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!( { "message": "Given index not valid. Try 'djia' or 'nasdaq'."})),
            )
        }
    };

    // 1) Get data for Dow Jones and prepare it for the response
    let quotes = fh_api
        .fetch_quotes_for_market(market)
        .await
        .expect("The market news to be fetched");

    let mut quote_gainers: Vec<SymbolQuoteFrontend> = quotes
        .iter()
        .filter(|x| x.delta_percent.is_sign_positive())
        .cloned()
        .collect();

    // Sort descending
    quote_gainers.sort_by(|a, b| b.delta_percent.partial_cmp(&a.delta_percent).unwrap());

    let mut quote_losers: Vec<SymbolQuoteFrontend> = quotes
        .iter()
        .filter(|x| x.delta_percent.is_sign_negative())
        .cloned()
        .collect();

    // Sort ascending
    quote_losers.sort_by(|a, b| a.delta_percent.partial_cmp(&b.delta_percent).unwrap());

    (
        StatusCode::OK,
        Json(json!( { "gainers": quote_gainers, "losers": quote_losers} )),
    )
}

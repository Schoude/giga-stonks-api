use crate::finnhub_api::lib::{Endpoint, FinnhubAPI};
use crate::finnhub_api::market_news::ArticleMarketNews;
use crate::finnhub_api::symbol_quote::SymbolQuoteFrontend;
use crate::indices::{DOW_JONES, NASDAQ};
use crate::AppState;
use axum::extract::{Path, State};
use axum::{http::StatusCode, Json};
use serde_json::{json, Value};
use std::cmp::Ordering;
use std::sync::Arc;

fn setup_finnhub_api(endpoint: Endpoint, api_token: &str) -> FinnhubAPI {
    let mut finnhub_api = FinnhubAPI::new(api_token);
    finnhub_api.endpoint(endpoint);
    finnhub_api
}

pub async fn get_market_news(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<Vec<ArticleMarketNews>>) {
    let fh_api = setup_finnhub_api(Endpoint::MarketNews, &state.api_token_finnhub);
    let articles = fh_api
        .fetch_market_news()
        .await
        .expect("The market news to be fetched");
    (StatusCode::OK, Json(articles))
}

pub async fn get_quotes_for_index(
    Path(index): Path<String>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<Value>) {
    let fh_api = setup_finnhub_api(Endpoint::Quote, &state.api_token_finnhub);

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

    let sentiment = match quote_gainers.len().cmp(&quote_losers.len()) {
        Ordering::Greater => "bullish",
        Ordering::Less => "bearish",
        Ordering::Equal => "neutral",
    };

    let gains_percentage_sum: f32 = quote_gainers.iter().map(|q| q.delta_percent).sum();
    let losses_percentage_sum: f32 = quote_losers.iter().map(|q| q.delta_percent).sum();
    let avg_percentage_gains = gains_percentage_sum / quote_gainers.len() as f32;
    let avg_percentage_losses = losses_percentage_sum / quote_losers.len() as f32;

    (
        StatusCode::OK,
        Json(json!( {
            "sentiment": sentiment,
            "avg_percentage_gains": avg_percentage_gains,
            "avg_percentage_losses": avg_percentage_losses,
            "gainers": quote_gainers,
            "losers": quote_losers,
        } )),
    )
}

use crate::finnhub_api::lib::{Endpoint, FinnhubAPI};
use crate::finnhub_api::market_news::ArticleMarketNews;
use crate::finnhub_api::symbol_quote::SymbolQuoteFrontend;
use crate::indices::DOW_JONES;
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

pub async fn get_quotes_overview() -> (StatusCode, Json<Value>) {
    let fh_api = setup_finnhub_api(Endpoint::Quote);

    // 1) Get data for Dow Jones and prepare it for the response
    let quotes_dj = fh_api
        .fetch_quotes_for_market(DOW_JONES)
        .await
        .expect("The market news to be fetched");

    let mut quote_dj_gainers: Vec<SymbolQuoteFrontend> = quotes_dj
        .iter()
        .filter(|x| x.dp.is_sign_positive())
        .cloned()
        .collect();

    // Sort descending
    quote_dj_gainers.sort_by(|a, b| b.dp.partial_cmp(&a.dp).unwrap());

    let mut quote_dj_losers: Vec<SymbolQuoteFrontend> = quotes_dj
        .iter()
        .filter(|x| x.dp.is_sign_negative())
        .cloned()
        .collect();

    // Sort ascending
    quote_dj_losers.sort_by(|a, b| a.dp.partial_cmp(&b.dp).unwrap());

    (
        StatusCode::OK,
        Json(json!({ "dowJones": { "gainers": quote_dj_gainers, "losers": quote_dj_losers} })),
    )
}

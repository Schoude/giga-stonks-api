use axum::{http::StatusCode, Json};

use crate::alphavantage_api::{
    lib::{AlphaVantageAPI, Endpoint},
    market_status::MarketStatusInfo,
};

fn setup_av_api(endpoint: Endpoint) -> AlphaVantageAPI {
    let av_api_token: String =
        std::env::var("ALPHA_VANTAGE_API_TOKEN").expect("ALPHA_VANTAGE_API_TOKEN must be set.");

    let mut av_api = AlphaVantageAPI::new(&av_api_token);
    av_api.endpoint(endpoint);
    av_api
}

pub async fn get_market_status() -> (StatusCode, Json<Vec<MarketStatusInfo>>) {
    let av_api = setup_av_api(Endpoint::MarketStatus);

    let markets_status = av_api
        .fetch_market_status()
        .await
        .expect("The market status to be fetched");
    (StatusCode::OK, Json(markets_status))
}

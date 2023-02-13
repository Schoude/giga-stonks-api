use alphavantage::{market_status::MarketStatusInfo, AlphaVantageAPI, Endpoint};
use axum::{http::StatusCode, Json};

pub async fn get_market_status() -> (StatusCode, Json<Vec<MarketStatusInfo>>) {
    let finnhub_api_token: String =
        std::env::var("FINNHUB_API_TOKEN").expect("FINNHUB_API_TOKEN must be set.");

    let mut av_api = AlphaVantageAPI::new(&finnhub_api_token);
    av_api.endpoint(Endpoint::MarketStatus);

    let markets_status = av_api
        .fetch_market_status()
        .await
        .expect("The market status to be fetched");
    (StatusCode::OK, Json(markets_status))
}

use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use crate::{
    alphavantage_api::{
        lib::{AlphaVantageAPI, Endpoint},
        market_status::MarketStatusInfo,
    },
    AppState,
};

fn setup_av_api(endpoint: Endpoint, api_token: &str) -> AlphaVantageAPI {
    let mut av_api = AlphaVantageAPI::new(api_token);
    av_api.endpoint(endpoint);
    av_api
}

pub async fn get_market_status(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<Vec<MarketStatusInfo>>) {
    let av_api = setup_av_api(Endpoint::MarketStatus, &state.api_token_alphavantage);
    let markets_status = av_api
        .fetch_market_status()
        .await
        .expect("The market status to be fetched");
    (StatusCode::OK, Json(markets_status))
}

use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    alphavantage_api::{
        lib::{AlphaVantageAPI, Endpoint},
        market_status::MarketStatusInfo,
        news_sentiment::{NewsSentimentFeedEntry, QueryNewsSentiment},
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

pub async fn get_news_sentiment(
    State(state): State<Arc<AppState>>,
    time_from: Query<QueryNewsSentiment>,
) -> (StatusCode, Json<Vec<NewsSentimentFeedEntry>>) {
    let time_from: QueryNewsSentiment = time_from.0;
    let av_api = setup_av_api(Endpoint::NewsSentiment, &state.api_token_alphavantage);
    let news_sentiment = av_api
        .fetch_news_sentiment(time_from.time_from)
        .await
        .expect("The news sentiment to be fetched");

    (StatusCode::OK, Json(news_sentiment))
}

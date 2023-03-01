use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

use crate::{
    alphavantage_api::{
        lib::{AlphaVantageAPI, Endpoint},
        market_status::MarketStatusInfo,
        news_sentiment::{NewsSentimentFeedEntry, QueryNewsSentiment, QueryNewsSentimentTicker},
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
) -> (StatusCode, Json<Value>) {
    let time_from: QueryNewsSentiment = time_from.0;
    let av_api = setup_av_api(Endpoint::NewsSentiment, &state.api_token_alphavantage);
    let news_sentiment = av_api
        .fetch_news_sentiment(time_from.time_from)
        .await
        .expect("The news sentiment to be fetched");

    let mut bullish: Vec<&NewsSentimentFeedEntry> = news_sentiment
        .iter()
        .filter(|feed_entry| feed_entry.overall_sentiment_label.contains("Bullish"))
        .collect();

    bullish.sort_by(|a, b| {
        b.overall_sentiment_score
            .partial_cmp(&a.overall_sentiment_score)
            .unwrap()
    });

    let mut bearish: Vec<&NewsSentimentFeedEntry> = news_sentiment
        .iter()
        .filter(|feed_entry| feed_entry.overall_sentiment_label.contains("Bearish"))
        .collect();

    bearish.sort_by(|a, b| {
        a.overall_sentiment_score
            .partial_cmp(&b.overall_sentiment_score)
            .unwrap()
    });

    (
        StatusCode::OK,
        Json(json!({
            "news_bullish": bullish,
            "news_bearish": bearish,
        })),
    )
}

pub async fn get_news_sentiment_ticker(
    State(state): State<Arc<AppState>>,
    query: Query<QueryNewsSentimentTicker>,
) -> (StatusCode, Json<Value>) {
    let time_from = query.0.time_from;
    let ticker= query.0.ticker;
    let av_api = setup_av_api(Endpoint::NewsSentiment, &state.api_token_alphavantage);
    let news_sentiment = av_api
        .fetch_news_sentiment_ticker(ticker, time_from)
        .await
        .expect("The news sentiment for a ticker to be fetched");

    let mut bullish: Vec<&NewsSentimentFeedEntry> = news_sentiment
        .iter()
        .filter(|feed_entry| feed_entry.overall_sentiment_label.contains("Bullish"))
        .collect();

    bullish.sort_by(|a, b| {
        b.overall_sentiment_score
            .partial_cmp(&a.overall_sentiment_score)
            .unwrap()
    });

    let mut bearish: Vec<&NewsSentimentFeedEntry> = news_sentiment
        .iter()
        .filter(|feed_entry| feed_entry.overall_sentiment_label.contains("Bearish"))
        .collect();

    bearish.sort_by(|a, b| {
        a.overall_sentiment_score
            .partial_cmp(&b.overall_sentiment_score)
            .unwrap()
    });

    (
        StatusCode::OK,
        Json(json!({
            "news_bullish": bullish,
            "news_bearish": bearish,
        })),
    )
}

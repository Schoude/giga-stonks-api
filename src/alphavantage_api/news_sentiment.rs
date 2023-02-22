use serde::{Deserialize, Serialize};

// Types for News Sentiment
#[derive(Deserialize, Debug, Serialize)]
struct TickerSentimentEntry {
    ticker: String,
    relevance_score: String,
    ticker_sentiment_score: String,
    ticker_sentiment_label: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct NewsSentimentFeedEntry {
    title: String,
    url: String,
    time_published: String,
    summary: String,
    banner_image: Option<String>,
    source: String,
    category_within_source: String,
    overall_sentiment_score: f32,
    overall_sentiment_label: String,
    ticker_sentiment: Vec<TickerSentimentEntry>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct NewsSentimentResponse {
    pub feed: Vec<NewsSentimentFeedEntry>,
}

#[derive(Deserialize)]
pub struct QueryNewsSentiment {
    pub time_from: String,
}

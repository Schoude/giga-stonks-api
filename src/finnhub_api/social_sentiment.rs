use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSentimentEntry {
    at_time: String,
    mention: u16,
    // positive_score: f32,
    // negative_score: f32,
    positive_mention: u16,
    negative_mention: u16,
    score: f32,
}

#[derive(Serialize, Deserialize)]
pub struct SocialSentimentResponse {
    reddit: Vec<SocialSentimentEntry>,
    twitter: Vec<SocialSentimentEntry>,
}

#[derive(Deserialize)]
pub struct QuerySocialSentiment {
    pub symbol: String,
    pub time_from: String,
}

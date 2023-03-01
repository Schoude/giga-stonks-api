use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct ArticleMarketNews {
    headline: String,
    category: String,
    datetime: u32,
    id: u32,
    image: String,
    source: String,
    summary: String,
    url: String,
}

#[derive(Deserialize)]
pub struct QueryCompanyNews {
    pub symbol: String,
    pub time_from: String,
    pub time_to: String,
}

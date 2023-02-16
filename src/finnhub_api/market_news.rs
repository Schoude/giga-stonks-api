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

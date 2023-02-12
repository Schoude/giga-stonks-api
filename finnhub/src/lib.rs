use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct ArticleMarketNews {
    headline: String,
    category: String,
    datetime: u32,
    id: u32,
    image: String,
    related: String,
    source: String,
    summary: String,
    url: String,
}

pub fn get_market_news(api_token: &str) -> Result<Vec<ArticleMarketNews>, Box<dyn Error>> {
    let mut market_news_url =
        String::from("https://finnhub.io/api/v1/news?category=general&token=");
    market_news_url.push_str(api_token);

    let response = ureq::get(&market_news_url).call()?.into_string()?;
    let atricles = serde_json::from_str::<Vec<ArticleMarketNews>>(&response)?;
    Ok(atricles)
}

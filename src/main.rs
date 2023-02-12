use dotenv::dotenv;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct ArticleMarketNews {
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

fn get_market_news(url: &str) -> Result<Vec<ArticleMarketNews>, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;
    let atricles = serde_json::from_str::<Vec<ArticleMarketNews>>(&response)?;
    Ok(atricles)
}

fn main() {
    dotenv().ok();
    let api_key: String =
        std::env::var("FINNHUB_API_TOKEN").expect("FINNHUB_API_TOKEN must be set.");
    let mut market_news_url =
        String::from("https://finnhub.io/api/v1/news?category=general&token=");
    market_news_url.push_str(api_key.as_str());
    let articles = get_market_news(&market_news_url).expect("Should load the market news articles");

    dbg!(articles);
}

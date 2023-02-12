use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum FinnhubError {
    #[error("Failed fetching the market news")]
    MarktedNewsRequestFailed(ureq::Error),
    #[error("Failed parsing to ArticleMarketNews")]
    ArticleMarketNewsParseFailed(serde_json::Error),

    // General errors
    #[error("Failed converting response to string")]
    FailedResponseToString(std::io::Error),
}

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

pub fn get_market_news(api_token: &str) -> Result<Vec<ArticleMarketNews>, FinnhubError> {
    let mut market_news_url =
        String::from("https://finnhub.io/api/v1/news?category=general&token=");
    market_news_url.push_str(api_token);

    let response = ureq::get(&market_news_url)
        .call()
        .map_err(|e| FinnhubError::MarktedNewsRequestFailed(e))?
        .into_string()
        .map_err(|e| FinnhubError::FailedResponseToString(e))?;

    let atricles = serde_json::from_str::<Vec<ArticleMarketNews>>(&response)
        .map_err(|e| FinnhubError::ArticleMarketNewsParseFailed(e))?;
    Ok(atricles)
}

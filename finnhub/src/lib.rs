use reqwest::Method;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://finnhub.io/api/v1/";

#[derive(thiserror::Error, Debug)]
pub enum FinnhubError<'a> {
    #[error("Failed fetching the market news")]
    MarktedNewsRequestFailed(#[from] ureq::Error),
    #[error("Failed fetching the market news")]
    AsyncRequestFailed(#[from] reqwest::Error),
    #[error("Failed parsing to ArticleMarketNews")]
    ArticleMarketNewsParseFailed(serde_json::Error),

    // General errors
    #[error("Failed converting response to string")]
    FailedResponseToString(#[from] std::io::Error),
    #[error("Failed to parse the URL")]
    URlParsing(#[from] url::ParseError),
    #[error("Bad Request")]
    BadRequest(&'a str),
}

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

#[derive(Debug)]
pub enum Endpoint {
    MarketNews,
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::MarketNews => "news?category=general".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct FinnhubAPI {
    api_key: String,
    endpoint: Endpoint,
}

impl FinnhubAPI {
    pub fn new(api_key: &str) -> FinnhubAPI {
        FinnhubAPI {
            api_key: api_key.to_string(),
            endpoint: Endpoint::MarketNews,
        }
    }

    pub fn endpoint(&mut self, endpoint: Endpoint) -> &mut FinnhubAPI {
        self.endpoint = endpoint;
        self
    }

    fn get_api_token(&self) -> String {
        format!("&token={}", self.api_key)
    }

    fn prepare_url(&self) -> String {
        format!(
            "{}{}{}",
            BASE_URL,
            self.endpoint.to_string(),
            self.get_api_token(),
        )
    }

    pub async fn fetch_market_news(&self) -> Result<Vec<ArticleMarketNews>, FinnhubError> {
        let url = self.prepare_url();
        let client = reqwest::Client::new();
        let req = client
            .request(Method::GET, url)
            .build()
            .map_err(|e| FinnhubError::AsyncRequestFailed(e))?;

        let res = client
            .execute(req)
            .await?
            .json()
            .await
            .map_err(|e| FinnhubError::AsyncRequestFailed(e))?;
        Ok(res)
    }
}

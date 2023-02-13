pub mod market_status;

use market_status::{MarketStatusInfo, MarketStatusResponse};
use reqwest::Method;

const BASE_URL: &str = "https://www.alphavantage.co/query?function=";

#[derive(thiserror::Error, Debug)]
pub enum AlphaVantageError<'a> {
    #[error("Failed fetching the market status")]
    MarketStatusRequestFailed(#[from] ureq::Error),
    #[error("Failed fetching the market status")]
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

#[derive(Debug)]
pub enum Endpoint {
    MarketStatus,
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::MarketStatus => "MARKET_STATUS".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct AlphaVantageAPI {
    api_key: String,
    endpoint: Endpoint,
}

impl AlphaVantageAPI {
    pub fn new(api_key: &str) -> AlphaVantageAPI {
        AlphaVantageAPI {
            api_key: api_key.to_string(),
            endpoint: Endpoint::MarketStatus,
        }
    }

    pub fn endpoint(&mut self, endpoint: Endpoint) -> &mut AlphaVantageAPI {
        self.endpoint = endpoint;
        self
    }

    fn get_api_key(&self) -> String {
        format!("&apikey={}", self.api_key)
    }

    fn prepare_url(&self) -> String {
        format!(
            "{}{}{}",
            BASE_URL,
            self.endpoint.to_string(),
            self.get_api_key(),
        )
    }

    pub async fn fetch_market_status(&self) -> Result<Vec<MarketStatusInfo>, AlphaVantageError> {
        let url = self.prepare_url();
        let client = reqwest::Client::new();
        let req = client
            .request(Method::GET, url)
            .build()
            .map_err(|e| AlphaVantageError::AsyncRequestFailed(e))?;

        let res: MarketStatusResponse = client
            .execute(req)
            .await?
            .json()
            .await
            .map_err(|e| AlphaVantageError::AsyncRequestFailed(e))?;
        Ok(res.markets)
    }
}

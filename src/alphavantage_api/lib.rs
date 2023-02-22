use super::{
    market_status::{MarketStatusInfo, MarketStatusResponse},
    news_sentiment::{NewsSentimentFeedEntry, NewsSentimentResponse},
};
use reqwest::Method;

const BASE_URL: &str = "https://www.alphavantage.co/query?function=";

#[derive(thiserror::Error, Debug)]
pub enum AlphaVantageError {
    #[error("Failed async fetching the market status")]
    AsyncRequestFailed(#[from] reqwest::Error),
}

#[derive(Debug)]
pub enum Endpoint {
    MarketStatus,
    NewsSentiment,
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::MarketStatus => "MARKET_STATUS".to_string(),
            Self::NewsSentiment => "NEWS_SENTIMENT".to_string(),
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

    /**
     *  Possibilities for url_add
     * 1) Endpoint::NewsSentiment: complete query string ?time_from=yyyymmddThhmmss
     */
    fn prepare_url(&self, url_add: Option<&str>) -> String {
        if let Some(url) = url_add {
            format!(
                "{}{}{}{}",
                BASE_URL,
                self.endpoint.to_string(),
                url,
                self.get_api_key(),
            )
        } else {
            format!(
                "{}{}{}",
                BASE_URL,
                self.endpoint.to_string(),
                self.get_api_key(),
            )
        }
    }

    pub async fn fetch_market_status(&self) -> Result<Vec<MarketStatusInfo>, AlphaVantageError> {
        let url = self.prepare_url(None);
        let client = reqwest::Client::new();
        let req = client
            .request(Method::GET, url)
            .build()
            .map_err(AlphaVantageError::AsyncRequestFailed)?;

        let mut res: MarketStatusResponse =
            client
                .execute(req)
                .await?
                .json()
                .await
                .unwrap_or(MarketStatusResponse {
                    endpoint: "API limit reached".to_string(),
                    markets: vec![],
                });

        res.markets
            .retain(|market| market.region == "United States" || market.region == "Germany");

        Ok(res.markets)
    }

    pub async fn fetch_news_sentiment(
        &self,
        time_from: String,
    ) -> Result<Vec<NewsSentimentFeedEntry>, AlphaVantageError> {
        let query = format!("&time_from={time_from}T0000");
        let url = self.prepare_url(Some(&query.as_str()));

        let client = reqwest::Client::new();
        let req = client
            .request(Method::GET, url)
            .build()
            .map_err(AlphaVantageError::AsyncRequestFailed)?;

        let res: NewsSentimentResponse = client
            .execute(req)
            .await?
            .json()
            .await
            .unwrap_or(NewsSentimentResponse { feed: vec![] });

        Ok(res.feed)
    }
}

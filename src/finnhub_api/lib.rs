use std::time::{SystemTime, UNIX_EPOCH};

use super::company_profile::CompanyProfile;
use super::market_news::ArticleMarketNews;
use super::social_sentiment::SocialSentimentResponse;
use super::symbol_quote::{SymbolQuote, SymbolQuoteExtended};
use reqwest::Method;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://finnhub.io/api/v1/";

#[derive(thiserror::Error, Debug)]
pub enum FinnhubError {
    #[error("Failed fetching the market news")]
    AsyncRequestFailed(#[from] reqwest::Error),
}

#[derive(Debug)]
pub enum Endpoint {
    MarketNews,
    Quote,
    CompanyProfile,
    SocialSentiment,
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::MarketNews => "news?category=general".to_string(),
            Self::Quote => "quote?symbol=".to_string(),
            Self::CompanyProfile => "stock/profile2?symbol=".to_string(),
            Self::SocialSentiment => "stock/social-sentiment".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct RateLimitInfo {
    pub ratelimit_remaining: String,
    pub ratelimit_reset: String,
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

    /**
     *  Possibilities for url_add
     * 1) Endpoint::Quote: a stock symbol like "AAPL" or "IBM"
     * 2) Endpoint::CompanyProfile: a stock symbol like "AAPL" or "IBM"
     */
    fn prepare_url(&self, url_add: Option<&str>) -> String {
        if let Some(url) = url_add {
            format!(
                "{}{}{}{}",
                BASE_URL,
                self.endpoint.to_string(),
                url,
                self.get_api_token(),
            )
        } else {
            format!(
                "{}{}{}",
                BASE_URL,
                self.endpoint.to_string(),
                self.get_api_token(),
            )
        }
    }

    pub async fn fetch_market_news(&self) -> Result<Vec<ArticleMarketNews>, FinnhubError> {
        let url = self.prepare_url(None);
        let client = reqwest::Client::new();
        let req = client
            .request(Method::GET, url)
            .build()
            .map_err(FinnhubError::AsyncRequestFailed)?;

        let res = client
            .execute(req)
            .await?
            .json()
            .await
            .map_err(FinnhubError::AsyncRequestFailed)?;
        Ok(res)
    }

    pub async fn fetch_quotes_for_market(
        &self,
        market: &'static [(&str, &str)],
    ) -> Result<Vec<SymbolQuoteExtended>, FinnhubError> {
        let client = reqwest::Client::new();

        let mut tasks = Vec::new();

        // Iterate over every symbol in the Dow Jones (30)
        for (symbol, name) in market.iter() {
            let client = client.clone();
            let url = self.prepare_url(Some(symbol));

            // Span a seperate task for each request with the cloned reqwest client and return
            // the expected data from the request
            let task = tokio::task::spawn(async move {
                let req = client
                    .request(Method::GET, url)
                    .build()
                    .expect("the request to be build.");

                let response = client
                    .execute(req)
                    .await
                    .expect("the request to be executed");
                let headers = response.headers().clone();
                let ratelimit_remaining = headers.get("X-Ratelimit-Remaining").unwrap();
                let ratelimit_reset = headers.get("X-Ratelimit-Reset").unwrap();
                let rate_limit_info = RateLimitInfo {
                    ratelimit_remaining: String::from(ratelimit_remaining.to_str().unwrap()),
                    ratelimit_reset: String::from(ratelimit_reset.to_str().unwrap()),
                };

                let quote = response.json().await.unwrap_or(SymbolQuote {
                    c: 0.0,
                    d: 0.0,
                    dp: 0.0,
                    h: 0.0,
                    l: 0.0,
                    o: 0.0,
                    pc: 0.0,
                    t: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                });

                SymbolQuoteExtended {
                    current_price: quote.c,
                    delta: quote.d,
                    delta_percent: quote.dp,
                    high: quote.h,
                    low: quote.l,
                    open: quote.o,
                    previous_close: quote.pc,
                    timestamp: quote.t,
                    symbol: symbol.to_string(),
                    name: name.to_string(),
                    rate_limit_info,
                }
            });

            tasks.push(task);
        }

        let mut quotes_fe: Vec<SymbolQuoteExtended> = vec![];

        // Loop through each task and await the result from the Future
        for task in tasks {
            let quote_fe = match task.await {
                Ok(q) => q,
                Err(_) => SymbolQuoteExtended {
                    current_price: 0.0,
                    delta: 0.0,
                    delta_percent: 0.0,
                    high: 0.0,
                    low: 0.0,
                    open: 0.0,
                    previous_close: 0.0,
                    timestamp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                    symbol: "".to_string(),
                    name: "".to_string(),
                    rate_limit_info: RateLimitInfo {
                        ratelimit_remaining: "0".to_string(),
                        ratelimit_reset: "0".to_string(),
                    },
                },
            };

            quotes_fe.push(quote_fe);
        }

        Ok(quotes_fe)
    }

    pub async fn fetch_company_profile(
        &self,
        symbol: &str,
    ) -> Result<CompanyProfile, FinnhubError> {
        let client = reqwest::Client::new();
        let url = self.prepare_url(Some(symbol));

        let req = client
            .request(Method::GET, url)
            .build()
            .expect("the request to be build.");

        let result = client
            .execute(req)
            .await
            .expect("the request to be executed")
            .json::<CompanyProfile>()
            .await
            .expect("the company profile response to be parsed to JSON");

        Ok(result)
    }

    pub async fn fetch_social_sentiment(
        &self,
        // ?symbol=XXX&?from=yyyy-mm-dd
        url_add: &str,
    ) -> Result<SocialSentimentResponse, FinnhubError> {
        let client = reqwest::Client::new();
        let url = self.prepare_url(Some(url_add));

        let req = client
            .request(Method::GET, url)
            .build()
            .expect("the request to be build.");

        let result = client
            .execute(req)
            .await
            .expect("the request to be executed")
            .json::<SocialSentimentResponse>()
            .await
            .expect("the social sentiment response to be parsed to JSON");

        Ok(result)
    }
}

use std::time::{SystemTime, UNIX_EPOCH};

use super::market_news::ArticleMarketNews;
use super::symbol_quote::{SymbolQuote, SymbolQuoteFrontend};
use reqwest::Method;

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
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::MarketNews => "news?category=general".to_string(),
            Self::Quote => "quote?symbol=".to_string(),
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

    /**
     *  Possibilitis for url_add
     * 1) Endpount::Quote: a stock symbol like "AAPL" or "IBM"
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
    ) -> Result<Vec<SymbolQuoteFrontend>, FinnhubError> {
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

                let quote: SymbolQuote = client
                    .execute(req)
                    .await
                    .expect("the request to be executed")
                    .json()
                    .await
                    .unwrap_or(SymbolQuote {
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

                SymbolQuoteFrontend {
                    current_price: quote.c,
                    delta: quote.d,
                    delta_percent: quote.d,
                    high: quote.h,
                    low: quote.l,
                    open: quote.o,
                    previous_close: quote.pc,
                    timestamp: quote.t,
                    symbol: symbol.to_string(),
                    name: name.to_string(),
                }
            });

            tasks.push(task);
        }

        let mut quotes_fe: Vec<SymbolQuoteFrontend> = vec![];

        // Loop through each task and await the result from the Future
        for task in tasks {
            let quote_fe = match task.await {
                Ok(q) => q,
                Err(_) => SymbolQuoteFrontend {
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
                },
            };

            quotes_fe.push(quote_fe);
        }

        Ok(quotes_fe)
    }
}

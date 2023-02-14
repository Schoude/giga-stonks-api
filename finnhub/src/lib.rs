pub mod market_news;
pub mod symbol_quote;

use market_news::ArticleMarketNews;
use reqwest::Method;
use symbol_quote::{SymbolQuote, SymbolQuoteFrontend};

const BASE_URL: &str = "https://finnhub.io/api/v1/";

#[derive(thiserror::Error, Debug)]
pub enum FinnhubError<'a> {
    #[error("Failed fetching the market news")]
    MarketNewsRequestFailed(#[from] ureq::Error),
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
            .map_err(|e| FinnhubError::AsyncRequestFailed(e))?;

        let res = client
            .execute(req)
            .await?
            .json()
            .await
            .map_err(|e| FinnhubError::AsyncRequestFailed(e))?;
        Ok(res)
    }

    pub async fn fetch_quotes_for_market(
        &self,
        market: &'static [(&str, &str)],
    ) -> Result<Vec<SymbolQuoteFrontend>, FinnhubError> {
        let client = reqwest::Client::new();

        let mut quotes_fe: Vec<SymbolQuoteFrontend> = vec![];

        // Iterate over every symbol in the Dow Jones (30)
        for (symbol, name) in market.iter() {
            let url = self.prepare_url(Some(symbol));
            let req = client
                .request(Method::GET, url)
                .build()
                .map_err(|e| FinnhubError::AsyncRequestFailed(e))?;

            let quote: SymbolQuote = client
                .execute(req)
                .await?
                .json()
                .await
                .map_err(|e| FinnhubError::AsyncRequestFailed(e))?;

            let quote_fe = SymbolQuoteFrontend {
                c: quote.c,
                d: quote.d,
                dp: quote.d,
                h: quote.h,
                l: quote.l,
                o: quote.o,
                pc: quote.pc,
                t: quote.t,
                symbol: symbol.to_string(),
                name: name.to_string(),
            };

            quotes_fe.push(quote_fe);
        }

        Ok(quotes_fe)
    }
}

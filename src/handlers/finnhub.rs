use crate::finnhub_api::lib::{Endpoint, FinnhubAPI};
use crate::finnhub_api::{market_news::ArticleMarketNews, symbol_quote::SymbolQuoteFrontend};
use crate::indices::DOW_JONES;
use axum::{http::StatusCode, Json};

fn setup_finnhub_api(endpoint: Endpoint) -> FinnhubAPI {
    let finnhub_api_token: String =
        std::env::var("FINNHUB_API_TOKEN").expect("FINNHUB_API_TOKEN must be set.");

    let mut finnhub_api = FinnhubAPI::new(&finnhub_api_token);
    finnhub_api.endpoint(endpoint);
    finnhub_api
}

pub async fn get_market_news() -> (StatusCode, Json<Vec<ArticleMarketNews>>) {
    let fh_api = setup_finnhub_api(Endpoint::MarketNews);
    let articles = fh_api
        .fetch_market_news()
        .await
        .expect("The market news to be fetched");
    (StatusCode::OK, Json(articles))
}

pub async fn get_quotes_overview() -> (StatusCode, Json<Vec<SymbolQuoteFrontend>>) {
    let fh_api = setup_finnhub_api(Endpoint::Quote);
    let mut quotes_market: Vec<SymbolQuoteFrontend> = vec![];

    let mut quotes_dj = fh_api
        .fetch_quotes_for_market(DOW_JONES)
        .await
        .expect("The market news to be fetched");

    quotes_market.append(&mut quotes_dj);

    (StatusCode::OK, Json(quotes_market))
}

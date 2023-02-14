use axum::{http::StatusCode, Json};
use finnhub::{market_news::ArticleMarketNews, Endpoint, FinnhubAPI};

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

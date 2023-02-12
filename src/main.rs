use dotenv::dotenv;
use finnhub::{Endpoint, FinnhubAPI};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let finnhub_api_token: String =
        std::env::var("FINNHUB_API_TOKEN").expect("FINNHUB_API_TOKEN must be set.");

    let mut fh_api = FinnhubAPI::new(&finnhub_api_token);
    fh_api.endpoint(Endpoint::MarketNews);

    let articles = fh_api
        .fetch_market_news()
        .await
        .expect("The market news to be fetched");

    dbg!(articles);
}

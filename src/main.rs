use dotenv::dotenv;
use finnhub::get_market_news;

fn main() {
    dotenv().ok();
    let finnhub_api_token: String =
        std::env::var("FINNHUB_API_TOKEN").expect("FINNHUB_API_TOKEN must be set.");
    let articles =
        get_market_news(&finnhub_api_token.as_str()).expect("Should load the market news articles");

    dbg!(articles);
}

# Giga Stonks API

This api summarizes some free stock market APIs. See the list below for the list of endpoints and where the original data comes from.

## API endpoints

### News, Sentiments and Insider Informations

| Data                      | Method | URL                                                                    | Data source(s) | Documentation                                                               |
| ------------------------- | ------ | ---------------------------------------------------------------------- | -------------- | --------------------------------------------------------------------------- |
| Market News               | `GET`  | `/api/v1/market-news`                                                  | Finnhub        | [Market News](https://finnhub.io/docs/api/market-news)                      |
| Company News              | `GET`  | `/api/v1/company-news?symbol=XXXX&time_from=yyyy-mm-dd&time_to=yyyy-mm-dd` | Finnhub        | [Company News](https://finnhub.io/docs/api/company-news)                    |
| News Sentiment            | `GET`  | `/api/v1/news-sentiment?time_from=yyyymmdd`                            | Alpha Vantage  | [News Sentiment](https://www.alphavantage.co/documentation/#news-sentiment) |
| News Sentiment for Ticker | `GET`  | `/api/v1/news-sentiment-ticker?ticker=XXXX&time_from=yyyymmdd`         | Alpha Vantage  | [News Sentiment](https://www.alphavantage.co/documentation/#news-sentiment) |
| Social Sentiment          | `GET`  | `/api/v1/social-sentiment?symbol=XXXX&time_from=yyyymmdd`              | Finnhub        | [Social Sentiment](https://finnhub.io/docs/api/social-sentiment)            |


### Market Information (general)

| Data          | Method | URL                     | Data source(s) | Documentation                                                             |
| ------------- | ------ | ----------------------- | -------------- | ------------------------------------------------------------------------- |
| Market Status | `GET`  | `/api/v1/market-status` | Alpha Vantage  | [Market Status](https://www.alphavantage.co/documentation/#market-status) |


### Quote Data

| Data                                                                                  | Method | URL                     | Data source(s) | Documentation                                                |
| ------------------------------------------------------------------------------------- | ------ | ----------------------- | -------------- | ------------------------------------------------------------ |
| Quote data for each stock in the given index: Dow Jones ('djia') or NASDAQ ('nasdaq') | `GET`  | `/api/v1/quotes/:index` | Finnhub        | [Single Quote for Symbol](https://finnhub.io/docs/api/quote) |

### Company Information

| Data            | Method | URL                            | Data source(s) | Documentation                                                     |
| --------------- | ------ | ------------------------------ | -------------- | ----------------------------------------------------------------- |
| Company Profile | `GET`  | `/api/v1/company-profile/AAPL` | Finnhub        | [Company Profile 2](https://finnhub.io/docs/api/company-profile2) |

# Giga Stonks API

This api summarizes some free stock market APIs. See the list below for the list of endpoints and where the original data comes from.

## API endpoints

### News, Sentiments and Insider Informations

| Data        | Method | URL                   | Data source(s) | Documentation                                          |
| ----------- | ------ | --------------------- | -------------- | ------------------------------------------------------ |
| Market News | `GET`  | `/api/v1/market-news` | Finnhub        | [Market News](https://finnhub.io/docs/api/market-news) |


### Market Information (general)

| Data          | Method | URL                     | Data source(s) | Documentation                                                             |
| ------------- | ------ | ----------------------- | -------------- | ------------------------------------------------------------------------- |
| Market Status | `GET`  | `/api/v1/market-status` | Alpha Vantage  | [Market Status](https://www.alphavantage.co/documentation/#market-status) |


### Quote Data

| Data                                                                | Method | URL                       | Data source(s) | Documentation                                                |
| ------------------------------------------------------------------- | ------ | ------------------------- | -------------- | ------------------------------------------------------------ |
| Quote data for each stock in the markets Dow Jones, S&P 500, NASDAQ | `GET`  | `/api/v1/quotes/overview` | Finnhub        | [Single Quote for Symbol](https://finnhub.io/docs/api/quote) |

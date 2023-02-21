use serde::{Deserialize, Serialize};

use super::lib::RateLimitInfo;

/**
 * Gets returned from Finnhub.
 *
 * c: current price
 * d: change in value (i.e. dollars)
 * dp: change in percent
 * h: high price of the day
 * l: low price of the day
 * o: open price of the day
 * pc: previous close price
 */
#[derive(Deserialize, Debug, Serialize)]
pub struct SymbolQuote {
    pub c: f32,
    pub d: f32,
    pub dp: f32,
    pub h: f32,
    pub l: f32,
    pub o: f32,
    pub pc: f32,
    pub t: u128,
}

/*
 * Intermediate state to transfer required data to the route handler.
 */
#[derive(Deserialize, Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct SymbolQuoteExtended {
    pub current_price: f32,
    pub delta: f32,
    pub delta_percent: f32,
    pub high: f32,
    pub low: f32,
    pub open: f32,
    pub previous_close: f32,
    pub timestamp: u128,
    pub symbol: String,
    pub name: String,
    pub rate_limit_info: RateLimitInfo,
}

/*
 * Gets returned to the frontend by the API.
 */
#[derive(Deserialize, Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct SymbolQuoteFrontend {
    pub current_price: f32,
    pub delta: f32,
    pub delta_percent: f32,
    pub high: f32,
    pub low: f32,
    pub open: f32,
    pub previous_close: f32,
    pub timestamp: u128,
    pub symbol: String,
    pub name: String,
}

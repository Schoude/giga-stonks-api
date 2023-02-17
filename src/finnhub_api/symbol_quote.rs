use serde::{Deserialize, Serialize};

/**
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
    pub t: u32,
}

#[derive(Deserialize, Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct SymbolQuoteFrontend {
    pub c: f32,
    pub d: f32,
    pub dp: f32,
    pub h: f32,
    pub l: f32,
    pub o: f32,
    pub pc: f32,
    pub t: u32,
    pub symbol: String,
    pub name: String,
}

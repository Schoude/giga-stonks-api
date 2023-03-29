use serde::{Deserialize, Serialize};

// https://www.alphavantage.co/documentation/#earnings-calendar
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Earning {
    symbol: String,
    name: String,
    report_date: String,
    fiscal_date_ending: String,
    pub estimate: Option<f32>,
    currency: String,
}

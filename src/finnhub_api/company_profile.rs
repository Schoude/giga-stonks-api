use serde::{Deserialize, Serialize};

// https://finnhub.io/docs/api/company-profile2
#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyProfile {
    country: String,
    currency: String,
    exchange: String,
    finnhub_industry: String,
    ipo: String,
    logo: String,
    name: String,
    ticker: String,
    weburl: String,
}

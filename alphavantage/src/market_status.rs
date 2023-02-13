use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct MarketStatusInfo {
    region: String,
    local_open: String,
    local_close: String,
    current_status: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct MarketStatusResponse {
    endpoint: String,
    pub markets: Vec<MarketStatusInfo>,
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct MarketStatusInfo {
    pub region: String,
    local_open: String,
    local_close: String,
    current_status: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct MarketStatusResponse {
    pub endpoint: String,
    pub markets: Vec<MarketStatusInfo>,
}

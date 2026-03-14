use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BinanceTrade {
    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "p")]
    pub price: String,

    #[serde(rename = "q")]
    pub quantity: String,

    #[serde(rename = "T")]
    pub trade_time: u64,

    #[serde(rename = "E")]
    pub event_time: u64,
}

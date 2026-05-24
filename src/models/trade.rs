use serde::Deserialize;
use bigdecimal::BigDecimal;

#[derive(Debug, Deserialize)]
pub struct BinanceTrade {
    pub e: String,

    #[serde(rename = "E")]
    pub event_time: u64,  // Event Time

    pub s: String,  // Symbol
    pub t: u64,     // Trade ID
    pub p: BigDecimal,  // Price
    pub q: BigDecimal,  // Quantity

    #[serde(rename = "T")]
    pub trade_time: u64,  // Trade Time

    pub m: bool,  // Is the buyer the market maker  true买方挂单，卖方主动吃单 false 卖方挂单 买方吃单

    #[serde(rename = "M")]
    pub ignore: bool,  
}
use serde::Deserialize;
use rust_decimal::Decimal;

pub trait ReturnSymbol {
    fn return_symbol(&self) -> &str;
}

#[derive(Debug, Deserialize)]
pub struct BinanceStreamTrade {
    pub stream: String,
    pub data: BinanceTrade,
}

#[derive(Debug, Deserialize)]
pub struct BinanceTrade {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,  // Event Time
    
    #[serde(rename = "s")]
    pub symbol: String,  // Symbol

    #[serde(rename = "t")]
    pub trade_id: u64,     // Trade ID

    #[serde(rename = "p")]
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,  // Price

    #[serde(rename = "q")]
    #[serde(with = "rust_decimal::serde::str")]
    pub quantity: Decimal,  // Quantity

    #[serde(rename = "T")]
    pub trade_time: u64,  // Trade Time

    #[serde(rename = "m")]
    pub is_buyer_market_maker: bool,  // Is the buyer the market maker  true买方挂单，卖方主动吃单 false 卖方挂单 买方吃单

    #[serde(rename = "M")]
    pub ignore: bool,  
}

impl ReturnSymbol for BinanceStreamTrade {
    fn return_symbol(&self) -> &str {
        &self.stream
    }
}

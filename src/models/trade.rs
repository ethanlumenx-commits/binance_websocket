use serde::Deserialize;
use rust_decimal::Decimal;

/// 交易数据提取 Trait
pub trait TradeDataExtractor {
    /// 返回交易对符号
    fn return_symbol(&self) -> &str;
    
    /// 获取价格
    fn get_price(&self) -> Decimal;
    
    /// 获取数量
    fn get_quantity(&self) -> Decimal;
    
    /// 是否为买方做市商（卖方主动）
    fn is_buyer_maker(&self) -> bool;
    
    /// 获取交易时间
    fn get_trade_time(&self) -> u64;
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

impl TradeDataExtractor for BinanceStreamTrade {
    fn return_symbol(&self) -> &str {
        &self.stream
    }
    
    fn get_price(&self) -> Decimal {
        self.data.price
    }
    
    fn get_quantity(&self) -> Decimal {
        self.data.quantity
    }
    
    fn is_buyer_maker(&self) -> bool {
        self.data.is_buyer_market_maker
    }
    
    fn get_trade_time(&self) -> u64 {
        self.data.trade_time
    }
}

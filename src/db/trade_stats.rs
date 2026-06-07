use chrono::{DateTime, Utc};
use sqlx::FromRow;
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TradeStats {
    pub id: i64,
    pub symbol: String,
    pub stat_time: DateTime<Utc>,
    pub trade_count: i32,
    pub buy_count: i32,
    pub sell_count: i32,
    pub buy_volume: Decimal,
    pub sell_volume: Decimal,
    pub avg_trade_size: Decimal,
    pub large_trade_count: i32,
    pub buy_ratio: Decimal,
    pub sell_ratio: Decimal,
    pub created_at: DateTime<Utc>,
}

pub struct NewTradeStats {
    pub symbol: String,
    pub stat_time: DateTime<Utc>,

    pub trade_count: i32,

    pub buy_count: i32,
    pub sell_count: i32,

    pub buy_volume: Decimal,
    pub sell_volume: Decimal,

    pub avg_trade_size: Decimal,

    pub large_trade_count: i32,

    pub buy_ratio: Decimal,
    pub sell_ratio: Decimal,
}
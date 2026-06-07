use rust_decimal::Decimal;
use crate::db::trade_stats::NewTradeStats;
use chrono::Utc;

pub trait OrderFlowStatsTrait {
    fn to_trade_stats(&self) -> NewTradeStats;
}

/// calculate order flow statistics
#[derive(Debug,Clone)]
pub struct OrderFlowStats {
    pub buy_volume: Decimal,
    pub sell_volume: Decimal,
    pub buy_count: u64,
    pub sell_count: u64,
    pub buy_ratio: Decimal,
    pub sell_ratio: Decimal,
    pub large_trade_count: u64,
    pub total_volume: Decimal,
    pub avg_trade_size: Decimal,
}

impl OrderFlowStats {
    /// initialize
    pub fn new()->Self{
        OrderFlowStats {
            buy_volume: Decimal::ZERO,
            sell_volume: Decimal::ZERO,
            buy_count: 0,
            sell_count: 0,
            buy_ratio: Decimal::ZERO,
            sell_ratio: Decimal::ZERO,
            large_trade_count: 0,
            total_volume: Decimal::ZERO,
            avg_trade_size: Decimal::ZERO,
        }
    }

    pub fn update(&mut self, price: Decimal, quantity: Decimal, is_buyer_maker: bool) {
        let total_price = price * quantity;

        self.total_volume += quantity;
        
        if is_buyer_maker {
            self.buy_volume += quantity;
            self.buy_count += 1;
        } else {
            self.sell_volume += quantity;
            self.sell_count += 1;
        }
        self.avg_trade_size = self.total_volume / Decimal::from(self.buy_count + self.sell_count);
        self.buy_ratio = self.buy_volume / self.total_volume * Decimal::from(100);
        self.sell_ratio = self.sell_volume / self.total_volume * Decimal::from(100);

        if total_price > Decimal::from(10000) {
            self.large_trade_count += 1;
        }
    }


    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn get_and_reset(&mut self) -> Self {
        let current_stats = self.clone();
        self.reset();
        current_stats
    }
}

impl OrderFlowStatsTrait for OrderFlowStats {
    fn to_trade_stats(&self) -> NewTradeStats {
        NewTradeStats {
            symbol: "symbol".to_string(),
            stat_time: Utc::now(),
            trade_count: (self.buy_count + self.sell_count) as i32,
            buy_count: self.buy_count as i32,
            sell_count: self.sell_count as i32,
            buy_volume: self.buy_volume,
            sell_volume: self.sell_volume,
            avg_trade_size: self.avg_trade_size,
            large_trade_count: self.large_trade_count as i32,
            buy_ratio: self.buy_ratio,
            sell_ratio: self.sell_ratio,
            
        }
    }
}

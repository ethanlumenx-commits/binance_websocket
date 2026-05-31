use rust_decimal::Decimal;

/// 订单流统计数据
#[derive(Debug, Clone)]
pub struct OrderFlowStats {
    /// 主动买入成交量
    pub buy_volume: Decimal,
    /// 主动卖出成交量
    pub sell_volume: Decimal,
    /// 主动买入成交笔数
    pub buy_count: u64,
    /// 主动卖出成交笔数
    pub sell_count: u64,
    /// 大单数量（超过阈值）
    pub large_trade_count: u64,
    /// 总成交量
    pub total_volume: Decimal,
    /// 平均单笔大小
    pub avg_trade_size: Decimal,
}

impl OrderFlowStats {
    pub fn new() -> Self {
        Self {
            buy_volume: Decimal::ZERO,
            sell_volume: Decimal::ZERO,
            buy_count: 0,
            sell_count: 0,
            large_trade_count: 0,
            total_volume: Decimal::ZERO,
            avg_trade_size: Decimal::ZERO,
        }
    }

    /// 计算买入占比 (0-100%)
    pub fn buy_ratio(&self) -> Option<Decimal> {
        let total = self.buy_volume + self.sell_volume;
        if total > Decimal::ZERO {
            Some((self.buy_volume / total) * Decimal::from(100))
        } else {
            None
        }
    }
}

/// 订单流分析器
#[derive(Debug)]
pub struct OrderFlowAnalyzer {
    /// 大单阈值（USDT）
    large_trade_threshold: Decimal,
    /// 当前窗口统计
    stats: OrderFlowStats,
}

impl OrderFlowAnalyzer {
    pub fn new(large_trade_threshold_usdt: f64) -> Self {
        Self {
            large_trade_threshold: Decimal::from_f64_retain(large_trade_threshold_usdt)
                .unwrap_or(Decimal::from(10000)),
            stats: OrderFlowStats::new(),
        }
    }

    /// 处理新的交易数据
    pub fn update(&mut self, price: Decimal, quantity: Decimal, is_buyer_maker: bool) {
        let trade_value = price * quantity;
        
        // 更新成交量
        self.stats.total_volume += quantity;
        
        // 判断买卖方向
        // is_buyer_maker = true: 卖方主动吃单 -> 卖盘
        // is_buyer_maker = false: 买方主动吃单 -> 买盘
        if is_buyer_maker {
            self.stats.sell_volume += quantity;
            self.stats.sell_count += 1;
        } else {
            self.stats.buy_volume += quantity;
            self.stats.buy_count += 1;
        }

        // 检测大单
        if trade_value >= self.large_trade_threshold {
            self.stats.large_trade_count += 1;
        }

        // 更新平均单笔大小
        let total_count = self.stats.buy_count + self.stats.sell_count;
        if total_count > 0 {
            self.stats.avg_trade_size = self.stats.total_volume / Decimal::from(total_count);
        }
    }

    /// 获取并重置统计（每秒调用）
    pub fn get_and_reset(&mut self) -> OrderFlowStats {
        let stats = self.stats.clone();
        self.stats = OrderFlowStats::new();
        stats
    }

    /// 获取当前统计（不重置）
    pub fn current_stats(&self) -> &OrderFlowStats {
        &self.stats
    }
}

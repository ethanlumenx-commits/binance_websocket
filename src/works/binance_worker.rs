use tokio::time::interval;
use tracing::{info, error};
use std::fmt::Debug;
use std::time::Duration;

use crate::strategies::order_flow::OrderFlowAnalyzer;
use crate::config::OrderFlowConfig;
use crate::models::trade::TradeDataExtractor;

/// 交易统计数据
#[derive(Debug)]
pub struct TradeStats {
    pub trade_count: u64,
}

impl TradeStats {
    pub fn new() -> Self {
        Self { trade_count: 0 }
    }
    
    pub fn increment(&mut self) {
        self.trade_count += 1;
    }
    
    pub fn reset(&mut self) {
        self.trade_count = 0;
    }
}

/// Binance 交易对工作器
pub struct BinanceWorker {
    symbol: String, 
    order_flow: OrderFlowAnalyzer, // 订单流分析
    stats: TradeStats, // 每秒交易统计
}

impl BinanceWorker {
    /// 创建工作器实例
    pub fn new(symbol: String, order_flow_config: &OrderFlowConfig) -> Self {
        Self {
            symbol,
            order_flow: OrderFlowAnalyzer::new(order_flow_config.large_trade_threshold_usdt),
            stats: TradeStats::new(),
        }
    }
    
    /// 处理单个交易数据
    pub fn process_trade<T>(&mut self, trade: &T) where T: TradeDataExtractor {
        self.stats.increment();
        
        // 更新订单流分析
        self.order_flow.update(
            trade.get_price(),
            trade.get_quantity(),
            trade.is_buyer_maker()
        );
    }

    /// 处理时间窗口tick
    pub fn handle_tick(&mut self) {
        let flow_stats = self.order_flow.get_and_reset();
        
        info!(
            "{} | Trades/sec: {} | Buy/Sell: {}/{} | AvgSize: {} | LargeTrades: {} | BuyRatio: {:.1}%",
            self.symbol,
            self.stats.trade_count,
            flow_stats.buy_count,
            flow_stats.sell_count,
            flow_stats.avg_trade_size,
            flow_stats.large_trade_count,
            flow_stats.buy_ratio().unwrap_or(rust_decimal::Decimal::ZERO)
        );
        
        self.stats.reset();
    }
}

/// 运行 Binance 工作器
pub async fn run_binance_worker<T>(
    mut work_rx: tokio::sync::mpsc::Receiver<T>,
    symbol: String,
    order_flow_config: OrderFlowConfig,
) where T: Debug + Send + Sync + TradeDataExtractor + 'static {  
    let mut worker = BinanceWorker::new(symbol.clone(), &order_flow_config);
    
    let mut interval = interval(Duration::from_secs(1));
    
    loop {
        tokio::select! {
            msg = work_rx.recv() => {
                match msg {
                    Some(stream_trade) => {
                        worker.process_trade(&stream_trade);
                    },
                    None => {
                        error!("Channel for symbol {} closed", symbol);
                        break;
                    },
                }
            },
            _ = interval.tick() => {
                worker.handle_tick();
            },
        }
    }
}

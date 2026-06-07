use tokio::time::interval;
use tracing::{info, error};
use std::fmt::Debug;
use std::time::Duration;
use sqlx::PgPool;


use crate::strategies::order_flow::OrderFlowStats;
use crate::models::trade::TradeDataExtractor;
use crate::strategies::order_flow::OrderFlowStatsTrait;
use crate::db::trade_stats::NewTradeStats;

use crate::server::trade_stats_server;

#[derive(Debug, Clone)]
pub struct TradeStats {
    /// 交易笔数
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

#[derive(Clone)]
pub struct BinanceWorker {
    symbol: String, 
    stats: TradeStats, // 每秒交易统计
    order_flow: OrderFlowStats,
}

impl BinanceWorker {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            stats: TradeStats::new(),
            order_flow: OrderFlowStats::new(),
        }
    }
    
    /// 处理单个交易数据
    pub fn process_trade<T>(&mut self, trade: &T) where T: TradeDataExtractor {
        self.stats.increment();
        
        self.order_flow.update(
            trade.get_price(),
            trade.get_quantity(),
            trade.is_buyer_maker()
        );
    }

    /// 处理时间窗口tick
    pub fn handle_tick(&mut self) {
        
        info!(
            "{} | 每秒交易次数: {} | 买/卖单数: {}/{} | 买/卖数量: {:.2}/{:.2} | 平均大小: {:.3} | 大交易: {} | 买占比: {:.1}% | 卖占比: {:.1}%",
            self.symbol,
            self.order_flow.buy_count + self.order_flow.sell_count,
            self.order_flow.buy_count,
            self.order_flow.sell_count,
            self.order_flow.buy_volume,
            self.order_flow.sell_volume,
            self.order_flow.avg_trade_size,
            self.order_flow.large_trade_count,
            self.order_flow.buy_ratio,
            self.order_flow.sell_ratio,
        );
        
        self.order_flow.reset();
    }
    
    /// 转换为交易统计数据
    pub fn to_trade_stats(&self) -> NewTradeStats {
        let mut stats = self.order_flow.to_trade_stats();
        stats.symbol = self.symbol.clone();
        stats
    }
}

/// 运行 Binance 工作器
pub async fn run_binance_worker<T>(
    mut work_rx: tokio::sync::mpsc::Receiver<T>,
    symbol: String,
    db_pool: &PgPool,
) where T: Debug + Send + Sync + TradeDataExtractor + 'static  {  
    let mut worker = BinanceWorker::new(symbol.clone());
    
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
                // 先获取统计数据，再重置
                let trade_stats = worker.to_trade_stats();
                
                // 只有当有交易数据时才存储
                if trade_stats.trade_count > 0 {
                    info!("Inserting trade stats for {}: {} trades", trade_stats.symbol, trade_stats.trade_count);
                    
                    worker.handle_tick();
                    
                    // 异步存储到数据库
                    let server = trade_stats_server::TradeStatsServer;
                    match server.insert_trade_stats_server(&trade_stats, db_pool).await {
                        Ok(_) => info!("Successfully inserted trade stats for {}", trade_stats.symbol),
                        Err(e) => error!("Failed to insert trade stats for {}: {}", trade_stats.symbol, e),
                    }
                } else {
                    worker.handle_tick();
                }
            },
        }
    }
}

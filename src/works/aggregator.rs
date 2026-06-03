use tracing::error;
use std::fmt::Debug;

use crate::config::AppConfig;
use crate::models::trade::TradeDataExtractor;
use crate::works::binance_worker::run_binance_worker;


/// Binance 聚合器（订单流分析）
pub async fn aggregator_worker_with_indicators<T>(
    mut main_rx: tokio::sync::mpsc::Receiver<T>,
    symbols: &[String],
    config: &AppConfig,
) where T: Debug + Send + Sync + TradeDataExtractor + 'static {
    let mut works_hash = std::collections::HashMap::new();
    
    // 为每个交易对创建独立的工作协程
    for symbol in symbols {
        let (work_tx, work_rx) = tokio::sync::mpsc::channel(config.websocket.channel_capacity);
        
        let symbol_clone = symbol.clone();
        
        tokio::spawn(async move {
            run_binance_worker(work_rx, symbol_clone).await;
        });
        
        works_hash.insert(symbol.clone(), work_tx);
    }

    // 分发交易数据
    while let Some(trade) = main_rx.recv().await {
        let symbol = trade.return_symbol().to_string();
        if let Some(work_tx) = works_hash.get(&symbol) {
            if let Err(e) = work_tx.send(trade).await {
                error!("Error sending trade to {}: {:?}", symbol, e);
            }
        }
    }
}

use tokio::time::interval;
use tracing::{info, error};
use std::{fmt::Debug, time::Duration};

use crate::models::trade::ReturnSymbol;

/// Receive trades from channel and aggregate them for each symbol
pub async fn aggregator_worker<T>(mut main_rx: tokio::sync::mpsc::Receiver<T>, symbols: &[String])
where T: Debug + Send + Sync + ReturnSymbol + 'static{
    
    let mut works_hash = std::collections::HashMap::new();
    
    // Create a channel for each symbol
    for symbol in symbols {
        let (work_tx, work_rx) = tokio::sync::mpsc::channel(1024);
        tokio::spawn(symbol_worker::<T>(work_rx, symbol.clone()));
        works_hash.insert(symbol.clone(), work_tx);
    }

    while let Some(trade) = main_rx.recv().await {
        let symbol = trade.return_symbol();
        if let Some(work_tx) = works_hash.get(symbol) {
            if let Err(e) = work_tx.send(trade).await {
                info!("Error sending trade: {:?}", e);
            }
        }
    }

}
#[derive(Debug)]
pub struct TradeAgg{
    pub trade_count:u64,
}

/// Receive trades from channel and aggregate them for each symbol
pub async fn symbol_worker<T>(mut work_rx: tokio::sync::mpsc::Receiver<T>, symbol: String) 
where T: Debug + Send + Sync + 'static + ReturnSymbol {

    let mut trade_agg = TradeAgg{trade_count: 0};
    let mut interval = interval(Duration::from_secs(1));
    
    loop{
        tokio::select! {
            msg = work_rx.recv() =>
                match msg {
                    Some(_trade) => {
                        trade_agg.trade_count += 1;
                    },
                    None => {
                        error!("Channel for symbol {} closed", symbol);
                    },
                },
            _ = interval.tick() => {
                info!("Tick for symbol {}, trade_per_interval_sec {}", symbol, trade_agg.trade_count);
                trade_agg.trade_count = 0;
            },
        }
    }
}

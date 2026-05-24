use tracing::info;

use crate::models::trade::BinanceTrade;
/// Receive trades from channel and aggregate them
pub async fn aggregator_worker(mut trade_rx: tokio::sync::mpsc::Receiver<BinanceTrade>) {
    while let Some(trade) = trade_rx.recv().await {
        info!("Received trade: {:?}", trade)
    }
}

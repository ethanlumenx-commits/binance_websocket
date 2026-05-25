use tracing::info;

use crate::models::trade::BinanceStreamTrade;
/// Receive trades from channel and aggregate them
pub async fn aggregator_worker(mut trade_rx: tokio::sync::mpsc::Receiver<BinanceStreamTrade>) {
    while let Some(trade) = trade_rx.recv().await {
        info!("Received trade: {:?}", trade)
    }
}

mod websockets;
mod works;
mod logger;
mod models;
use crate::models::trade::BinanceStreamTrade;

#[tokio::main]
async fn main() {
    let _guard = logger::init_logger();
    let (main_tx, main_rx) = tokio::sync::mpsc::channel(32);
    let symbols = vec![
        "btcusdt@trade".to_string(), 
        "ethusdt@trade".to_string(), 
        "bnbusdt@trade".to_string(),
    ];
    tokio::spawn(websockets::binance_websocket::<BinanceStreamTrade>(main_tx, symbols.clone()));
    
    works::aggregator_worker::<BinanceStreamTrade>(main_rx, &symbols).await;

}

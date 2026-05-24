use tokio_tungstenite::{connect_async};
use futures::{StreamExt};
use tracing::{info, error};

use crate::models::trade::BinanceTrade;

use crate::logger;


async fn binance_websocket(tx: tokio::sync::mpsc::Sender<BinanceTrade>) {
    let _guard = logger::init_logger();
    let url = "wss://data-stream.binance.vision/ws/btcusdt@trade";
    let (socket, response) = connect_async(url).await.unwrap();

    info!("Connected status: {}", response.status());

    let (_, mut read) = socket.split();
    
    while let Some(msg) = read.next().await {
        match msg {
            Ok(msg) => {
                if msg.is_text() {
                    println!("{}", msg.to_string());
                    let trade: BinanceTrade = serde_json::from_str(&msg.to_string()).expect("Failed to parse JSON");
                    if let Err(e) = tx.send(trade).await { error!("Error sending trade: {}", e); };
                }
            },
            Err(e) => error!("Error: {}", e),
        }
    }
}

#[tokio::test]
async fn binance_websocket_test() {
    binance_websocket(tokio::sync::mpsc::channel(32).0).await;
}

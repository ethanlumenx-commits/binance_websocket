use tokio_tungstenite::{connect_async};
use futures::{StreamExt};
use tracing::{info, error};

use crate::models::trade::BinanceTrade;

/// Binance websocket -> Trade(tx) -> channel -> Aggregator
pub async fn binance_websocket(tx: tokio::sync::mpsc::Sender<BinanceTrade>) {
    let url = "wss://data-stream.binance.vision/ws/btcusdt@trade";
    loop {
        let (socket, response) = match connect_async(url).await {
            Ok(res) => res,
            Err(e) => {
                error!("Error to connect: {}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                continue;
            },
        };

        info!("Connected status: {}", response.status());
        let (_, mut read) = socket.split();
        while let Some(msg) = read.next().await {
            // msg is err
            let msg =match msg{
                Ok(msg) => msg,
                Err(e) => {
                    error!("Error receiving message: {}", e);
                    continue;
                },
            };

            // msg is not text
            if !msg.is_text() {
                continue;
            }

            // failed to parse message
            let text = match msg.to_text() {
                Ok(text) => text,
                Err(e) => {
                    error!("Error converting message to text: {}", e);
                    continue;
                },
            };

            //failed to Trade
            let trade: BinanceTrade = match serde_json::from_str(text) {
                Ok(trade) => trade,
                Err(e) => {
                    error!("Error parsing JSON: {}", e);
                    continue;
                },
            };
            
            // failed to send trade
            if let Err(e) = tx.send(trade).await { 
                error!("Error sending trade: {}, you must to restart the program and channel", e); 
                break;
            };

        };
        error!("Connection closed,sleep for 3 seconds");
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }

}

#[tokio::test]
async fn binance_websocket_test() {
    binance_websocket(tokio::sync::mpsc::channel(32).0).await;
}

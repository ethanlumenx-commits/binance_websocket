use tokio_tungstenite::{connect_async};
use futures::{StreamExt};
use tracing::{info, error};
use serde::Deserialize;


/// Binance websocket -> Trade(tx) -> channel -> Aggregator
pub async fn binance_websocket<T>(tx: tokio::sync::mpsc::Sender<T>, symbols: Vec<String>)
 where T: for<'de> Deserialize<'de> + Send + Sync + 'static {
    // let url = "wss://data-stream.binance.vision/ws/btcusdt@trade";
    let url =
        format!("wss://data-stream.binance.vision/stream?streams={}", symbols.join("/"));
    loop {
        let (socket, response) = match connect_async(&url).await {
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
            let streamtrade: T = match serde_json::from_str::<T>(text) {
                Ok(trade) => trade,
                Err(e) => {
                    error!("Error parsing JSON: {}", e);
                    error!("parsing JSON: {}", text);
                    continue;
                },
            };
            
            // failed to send trade
            if let Err(e) = tx.send(streamtrade).await { 
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
    use crate::logger;
    let _guard = logger::init_logger();
    use crate::models::trade::BinanceStreamTrade;
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);
    tokio::spawn(async move {
        while let Some(trade) = rx.recv().await {
            info!("recv trade: {:?}", trade);
        }
    });

    binance_websocket::<BinanceStreamTrade>(tx, vec!["btcusdt@trade".to_string()]).await;
}

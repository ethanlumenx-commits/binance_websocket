mod websockets;
mod works;
mod logger;
mod models;

#[tokio::main]
async fn main() {
    let _guard = logger::init_logger();
    let (main_tx, main_rx) = tokio::sync::mpsc::channel(32);

    tokio::spawn(websockets::binance_websocket(main_tx));
    works::aggregator_worker(main_rx).await;

}

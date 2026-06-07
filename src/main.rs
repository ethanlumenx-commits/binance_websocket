use binance_websocket::websockets;
use binance_websocket::works;
use binance_websocket::logger;
use binance_websocket::routes;
use binance_websocket::models::trade::BinanceStreamTrade;
use binance_websocket::config::AppConfig;
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    let config = AppConfig::load();
    let _guard = logger::init_logger();

    let pg_pool = PgPool::connect(&config.database.url).await.expect("Failed to connect to Postgres");
    
    let (main_tx, main_rx) = tokio::sync::mpsc::channel(config.websocket.channel_capacity);
    
    let symbols = vec![
        "btcusdt@trade".to_string(), 
        "ethusdt@trade".to_string(), 
        "bnbusdt@trade".to_string(),
    ];
    
    tokio::spawn(websockets::binance_websocket::<BinanceStreamTrade>(
        main_tx, 
        symbols.clone()
    ));
    
    let pg_pool_clone = pg_pool.clone();
    tokio::spawn(async move {
        works::aggregator_worker_with_indicators::<BinanceStreamTrade>(main_rx, &symbols, &config, &pg_pool_clone).await;
    });
    
    // 启动 HTTP 服务器
    let app = routes::create_router(pg_pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

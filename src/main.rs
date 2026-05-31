use binance_websocket::websockets;
use binance_websocket::works;
use binance_websocket::logger;
use binance_websocket::models::trade::BinanceStreamTrade;
use binance_websocket::config::AppConfig;

#[tokio::main]
async fn main() {
    // 加载配置
    let config = AppConfig::load();
    
    // 初始化日志
    let _guard = logger::init_logger();
    
    // 创建 channel
    let (main_tx, main_rx) = tokio::sync::mpsc::channel(config.websocket.channel_capacity);
    
    // 配置交易对
    let symbols = vec![
        "btcusdt@trade".to_string(), 
        "ethusdt@trade".to_string(), 
        "bnbusdt@trade".to_string(),
    ];
    
    // 启动 WebSocket 接收器
    tokio::spawn(websockets::binance_websocket::<BinanceStreamTrade>(
        main_tx, 
        symbols.clone()
    ));
    
    // 启动聚合工作器（订单流分析）
    works::aggregator_worker_with_indicators::<BinanceStreamTrade>(main_rx, &symbols, &config).await;
}

use serde::Deserialize;

/// 系统配置
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub websocket: WebSocketConfig,
    pub logging: LoggingConfig,
}

/// WebSocket 配置
#[derive(Debug, Clone, Deserialize)]
pub struct WebSocketConfig {
    pub base_url: String,
    pub reconnect_delay_secs: u64,
    pub channel_capacity: usize,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            base_url: "wss://data-stream.binance.vision".to_string(),
            reconnect_delay_secs: 3,
            channel_capacity: 1024,
        }
    }
}


/// 日志配置
#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file_path: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            file_path: "logs/websockets.log".to_string(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            websocket: WebSocketConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl AppConfig {
    /// 从环境变量和默认值加载配置
    pub fn load() -> Self {
        // 这里可以从配置文件或环境变量加载
        // 目前使用默认配置
        Self::default()
    }
}

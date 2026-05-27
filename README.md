# Binance WebSocket Trading Bot

一个基于 Rust 开发的币安（Binance）WebSocket 实时交易数据接收和处理系统。

## 项目简介

本项目是一个高性能的加密货币交易数据监听工具，通过币安的 WebSocket API 实时接收多个交易对的成交数据，并进行聚合统计处理。采用异步架构设计，支持多交易对并发处理。

## 核心功能

### 1. **实时 WebSocket 数据接收**
- 连接币安官方 WebSocket 数据流 (`wss://data-stream.binance.vision`)
- 支持同时订阅多个交易对（如 BTC/USDT, ETH/USDT, BNB/USDT 等）
- 自动重连机制：连接断开后自动在 3 秒后重新连接
- 泛型设计：支持不同类型的数据模型，具有良好的扩展性

### 2. **多交易对并发处理**
- 使用 Tokio 异步运行时实现高并发
- 为每个交易对创建独立的工作协程（worker）
- 通过 channel 进行模块间通信，实现解耦

### 3. **交易数据聚合统计**
- 按交易对分类处理数据
- 每秒统计每个交易对的成交笔数
- 实时输出交易量统计信息

### 4. **完善的日志系统**
- 使用 `tracing` 框架记录运行日志
- 同时输出到控制台和日志文件（`logs/websockets.log`）
- 支持通过 `RUST_LOG` 环境变量配置日志级别
- 日志文件采用追加模式，保留历史记录

### 5. **健壮的错误处理**
- WebSocket 连接错误自动重试
- JSON 解析失败时记录错误并继续处理
- Channel 发送失败时提供详细错误信息

## 技术栈

- **异步运行时**: [Tokio](https://tokio.rs/) - 完整的异步运行时支持
- **WebSocket 客户端**: [tokio-tungstenite](https://github.com/snapview/tokio-tungstenite) - 异步 WebSocket 实现
- **序列化**: [serde](https://serde.rs/) + [serde_json](https://github.com/serde-rs/json) - JSON 数据处理
- **高精度小数**: [rust_decimal](https://github.com/paupino/rust-decimal) - 精确的价格和数量计算
- **日志系统**: [tracing](https://github.com/tokio-rs/tracing) + [tracing-subscriber](https://docs.rs/tracing-subscriber) - 结构化日志

## 项目结构

```
binance_websocket/
├── src/
│   ├── main.rs              # 程序入口，初始化并启动各模块
│   ├── lib.rs               # 库模块声明
│   ├── logger.rs            # 日志系统初始化
│   ├── models/
│   │   ├── mod.rs           # 模型模块导出
│   │   └── trade.rs         # 交易数据模型定义
│   ├── websockets/
│   │   ├── mod.rs           # WebSocket 模块导出
│   │   └── binance.rs       # 币安 WebSocket 连接和数据接收
│   └── works/
│       ├── mod.rs           # 工作模块导出
│       └── aggregator.rs    # 数据聚合和工作协程
├── logs/
│   └── websockets.log       # 日志文件
├── Cargo.toml               # 项目依赖配置
└── README.md                # 项目说明文档
```

## 快速开始

### 环境要求

- Rust 1.75+ (edition 2024)
- Cargo 包管理器

### 安装依赖

```bash
cargo build
```

### 运行程序

```bash
# 默认运行（监听 BTC、ETH、BNB 三个交易对）
cargo run

# 自定义日志级别
RUST_LOG=debug cargo run
```

### 配置交易对

在 `src/main.rs` 中修改 `symbols` 向量：

```rust
let symbols = vec![
    "btcusdt@trade".to_string(), 
    "ethusdt@trade".to_string(), 
    "solusdt@trade".to_string(),  // 添加更多交易对
];
```

支持的 stream 类型：
- `<symbol>@trade` - 实时成交数据
- `<symbol>@depth` - 深度数据
- `<symbol>@kline_<interval>` - K线数据

## 架构设计

### 数据流

```
Binance WebSocket
       ↓
binance_websocket() [接收原始数据]
       ↓
   Channel (mpsc)
       ↓
aggregator_worker() [分发到各交易对]
       ↓
   Channel (per symbol)
       ↓
symbol_worker() [聚合统计]
       ↓
   日志输出
```

### 关键组件

1. **binance_websocket**: 泛型异步函数，负责建立 WebSocket 连接并接收数据
2. **aggregator_worker**: 根据交易对符号将数据分发到对应的工作协程
3. **symbol_worker**: 针对单个交易对进行数据统计（每秒成交笔数）
4. **ReturnSymbol trait**: 定义从数据中提取交易对符号的接口

## 扩展开发

### 添加新的数据类型

1. 在 `src/models/trade.rs` 中定义新的数据结构
2. 实现 `ReturnSymbol` trait
3. 在主函数中使用新类型调用 `binance_websocket`

### 自定义聚合逻辑

修改 `symbol_worker` 函数中的聚合逻辑，可以添加：
- 价格统计分析
- 成交量累计
- 买卖方向统计
- 自定义时间窗口聚合

## 注意事项

- 确保网络连接稳定，程序会自动处理断线重连
- 日志文件会持续增长，建议定期清理或轮转
- 订阅过多交易对可能影响性能，根据实际情况调整
- Channel 容量可根据数据量调整（当前 main channel: 32, worker channel: 1024）

## 许可证

本项目仅供学习和研究使用。

## 贡献

欢迎提交 Issue 和 Pull Request！

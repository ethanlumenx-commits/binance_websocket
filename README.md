# Binance WebSocket Order Flow Analyzer

基于 Rust 开发的币安（Binance）WebSocket 实时订单流分析系统。

## 项目简介

本项目是一个高性能的加密货币市场微观结构分析工具，通过币安的 WebSocket API 实时接收多个交易对的成交数据，并进行订单流分析，判断买卖力量对比。采用异步架构设计，支持多交易对并发处理。


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

### 3. **订单流分析** ⭐ 核心功能
- **买卖方向统计**: 主动买入 vs 主动卖出比例
  - `is_buyer_maker = false`: 买方主动（以卖一价成交）→ 买盘强劲
  - `is_buyer_maker = true`: 卖方主动（以买一价成交）→ 卖盘强劲
- **大单检测**: 识别鲸鱼交易（可配置阈值，默认 10,000 USDT）
- **交易频率分析**: 实时统计每秒成交情况
- **买卖压力指标**: 计算买入占比（0-100%）
  - > 50%: 买方强势
  - < 50%: 卖方强势
- **实时输出**: 每秒输出详细的订单流统计数据

### 4. **完善的日志系统**
- 使用 `tracing` 框架记录运行日志
- 同时输出到控制台和日志文件（`logs/websockets.log`）
- 支持通过 `RUST_LOG` 环境变量配置日志级别
- 日志文件采用追加模式，保留历史记录

### 5. **配置管理系统**
- 统一的配置结构 `AppConfig`，避免硬编码
- 支持 WebSocket、订单流分析、日志等多个维度的配置
- 大单阈值可灵活调整

### 6. **工程化代码质量**
- ✅ 零编译器警告
- ✅ 无魔法数字（所有常量都有语义化名称）
- ✅ 无代码重复（遵循 DRY 原则）
- ✅ 模块化设计（单一职责原则）
- ✅ 配置驱动（易于调整和测试）

## 技术栈

### 核心框架
- **异步运行时**: [Tokio](https://tokio.rs/) - 完整的异步运行时支持
- **WebSocket 客户端**: [tokio-tungstenite](https://github.com/snapview/tokio-tungstenite) - 异步 WebSocket 实现
- **序列化**: [serde](https://serde.rs/) + [serde_json](https://github.com/serde-rs/json) - JSON 数据处理

### 数据处理
- **高精度小数**: [rust_decimal](https://github.com/paupino/rust-decimal) - 精确的价格和数量计算

### 工程化工具
- **日志系统**: [tracing](https://github.com/tokio-rs/tracing) + [tracing-subscriber](https://docs.rs/tracing-subscriber) - 结构化日志
- **配置管理**: serde derive - 配置文件解析

## 项目结构

```
binance_websocket/
├── src/
│   ├── main.rs              # 程序入口，初始化并启动各模块
│   ├── lib.rs               # 库模块声明
│   ├── config.rs            # 配置管理系统
│   ├── errors.rs            # 统一错误处理
│   ├── logger.rs            # 日志系统初始化
│   ├── models/
│   │   ├── mod.rs           # 模型模块导出
│   │   └── trade.rs         # 交易数据模型 + TradeDataExtractor Trait
│   ├── websockets/
│   │   ├── mod.rs           # WebSocket 模块导出
│   │   └── binance.rs       # 币安 WebSocket 连接和数据接收
│   ├── works/
│   │   ├── mod.rs           # 工作模块导出
│   │   ├── aggregator.rs    # 数据分发器（泛型）
│   │   └── binance_worker.rs # 交易对工作器（泛型 + 订单流分析）
│   └── strategies/
│       ├── mod.rs           # 策略模块导出
│       └── order_flow.rs    # 订单流分析器（核心）
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

### 配置大单阈值

在 `src/config.rs` 中修改 `OrderFlowConfig`：

```rust
pub struct OrderFlowConfig {
    pub large_trade_threshold_usdt: f64,  // 默认 10000.0
}
```

## 架构设计

### 数据流

```
Binance WebSocket
       ↓
binance_websocket() [接收原始数据]
       ↓
   Channel (mpsc)
       ↓
aggregator_worker() [按交易对分发]
       ↓
   Channel (per symbol)
       ↓
binance_worker.run() [订单流分析]
       ├─► process_trade()        ← 处理每笔交易
       │     └─► update()         ← 更新订单流统计
       └─► handle_tick()          ← 每秒输出统计
       ↓
   日志输出
```

### 关键组件

1. **TradeDataExtractor Trait**: 统一的数据提取接口，支持泛型
2. **binance_websocket**: 泛型异步函数，负责建立 WebSocket 连接并接收数据
3. **aggregator_worker**: 根据交易对符号将数据分发到对应的工作协程
4. **BinanceWorker**: 封装的交易对工作器，包含订单流分析器
5. **OrderFlowAnalyzer**: 订单流分析引擎，统计买卖力量对比

### 设计模式

- **Trait 抽象**: `TradeDataExtractor` 实现数据访问的统一接口
- **泛型编程**: 工作器支持任意实现了 Trait 的数据类型
- **单一职责**: 每个模块只做一件事，易于维护和测试
- **观察者模式**: Channel 实现数据订阅和分发

## 扩展开发

### 添加新的数据字段

1. 在 `models/trade.rs` 的 `BinanceTrade` 中添加字段
2. 在 `TradeDataExtractor` Trait 中添加新方法
3. 在 `impl TradeDataExtractor for BinanceStreamTrade` 中实现
4. 在 `order_flow.rs` 中使用新字段

### 自定义聚合逻辑

修改 `binance_worker.rs` 中的处理逻辑，可以添加：
- 价格统计分析（最高价、最低价、均价）
- 成交量累计和分布
- 自定义时间窗口聚合
- 数据持久化（写入数据库）

### 添加数据持久化

1. 添加 SQLx 或 Diesel 依赖
2. 创建数据库 schema
3. 在 `binance_worker` 中定期保存数据
4. 实现历史数据查询接口

## 注意事项

- 确保网络连接稳定，程序会自动处理断线重连
- 日志文件会持续增长，建议定期清理或轮转
- 订阅过多交易对可能影响性能，根据实际情况调整
- Channel 容量可根据数据量调整（当前 main channel: 1024, worker channel: 1024）
- **风险管理**: 本系统仅用于学习和研究，不构成投资建议，实盘交易需谨慎

## 核心技能

本项目展示了以下核心技能：

✅ **Rust 高级特性**: 异步编程、泛型、Trait、生命周期  
✅ **系统设计能力**: 模块化架构、Channel 通信、并发控制  
✅ **工程化实践**: 配置管理、错误处理、日志系统、代码规范  
✅ **量化金融知识**: 订单流分析、市场微观结构、买卖力量对比  
✅ **设计模式应用**: Trait 抽象、泛型编程、单一职责  
✅ **性能优化意识**: 简洁高效、零拷贝、内存安全  

## 许可证

本项目仅供学习和研究使用。

## 贡献

欢迎提交 Issue 和 Pull Request！


use crate::db::trade_stats::{TradeStats,NewTradeStats};
use sqlx::PgPool;
use anyhow;


pub async fn insert_trade_stats(trade_stats: &NewTradeStats, db_pool: &PgPool) -> anyhow::Result<()> {
    tracing::info!("Attempting to insert trade stats for symbol: {}, trade_count: {}", 
                   trade_stats.symbol, trade_stats.trade_count);
    
    let result = sqlx::query(
        r#"
        INSERT INTO trade_stats
         (symbol, stat_time, trade_count, buy_count, sell_count, 
         buy_volume, sell_volume, avg_trade_size, large_trade_count, 
         buy_ratio, sell_ratio, created_at) 
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"#)
        .bind(&trade_stats.symbol)
        .bind(&trade_stats.stat_time)
        .bind(&trade_stats.trade_count)
        .bind(&trade_stats.buy_count)
        .bind(&trade_stats.sell_count)
        .bind(&trade_stats.buy_volume)
        .bind(&trade_stats.sell_volume)
        .bind(&trade_stats.avg_trade_size)
        .bind(&trade_stats.large_trade_count)
        .bind(&trade_stats.buy_ratio)
        .bind(&trade_stats.sell_ratio)
        .bind(chrono::Utc::now())
        .execute(db_pool)
        .await;
    
    match result {
        Ok(_) => {
            tracing::info!("Successfully inserted trade stats for {}", trade_stats.symbol);
            Ok(())
        }
        Err(e) => {
            tracing::error!("Database insert error for {}: {:?}", trade_stats.symbol, e);
            Err(anyhow::anyhow!(e))
        }
    }
}

pub async fn fetch_trade_stats(db_pool: &PgPool) -> anyhow::Result<Vec<TradeStats>> {
    let trade_stats = sqlx::query_as!(
        TradeStats,
        r#"
        SELECT id, symbol, stat_time, trade_count, buy_count, sell_count, 
        buy_volume, sell_volume, avg_trade_size, large_trade_count, 
        buy_ratio, sell_ratio, created_at 
        FROM trade_stats"#)
    .fetch_all(db_pool)
    .await?;
    Ok(trade_stats)
}

#[tokio::test]
async fn insert() {
    use chrono::prelude::*;
    use crate::db::trade_stats::{NewTradeStats};
    use sqlx::postgres::PgPool;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    let database_url = "postgres://postgres:amo@localhost:5432/postgres";
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    let trade_stats = NewTradeStats {
        symbol: "BTCUSDT".to_string(),
        stat_time: Utc::now(),
        trade_count: 100,
        buy_count: 50,
        sell_count: 50,
        buy_volume: Decimal::from_str("10.0").unwrap(),
        sell_volume: Decimal::from_str("10.0").unwrap(),
        avg_trade_size: Decimal::from_str("0.1").unwrap(),
        large_trade_count: 10,
        buy_ratio: Decimal::from_str("0.5").unwrap(),
        sell_ratio: Decimal::from_str("0.5").unwrap(),
    };
    insert_trade_stats(&trade_stats, &db_pool).await.unwrap();
}

#[tokio::test]
async fn fetch() {
    use sqlx::postgres::PgPool;

    let database_url = "postgres://postgres:amo@localhost:5432/postgres";
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    let trade_stats = fetch_trade_stats(&db_pool).await.unwrap();
    println!("{:?}", trade_stats);
}

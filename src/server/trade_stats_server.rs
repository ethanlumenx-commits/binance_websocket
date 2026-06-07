use crate::repo::insert_trade_stats::{insert_trade_stats};
use crate::db::trade_stats::NewTradeStats;
use sqlx::PgPool;
use anyhow::Result;

pub struct TradeStatsServer;

impl TradeStatsServer {
    pub async fn insert_trade_stats_server(&self, trade_stats: &NewTradeStats, db_pool: &PgPool) -> Result<()> {
        insert_trade_stats(trade_stats, db_pool).await.map_err(|e| anyhow::anyhow!(e.to_string()))?;
        Ok(())
    }
}
 
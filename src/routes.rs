use axum::{
    extract::State,
    response::Json,
    routing::get,
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::repo::insert_trade_stats::fetch_trade_stats;

pub fn create_router(db: PgPool) -> Router {
    Router::new()
        .route("/health", get(|| async { Json(serde_json::json!({"status": "ok"})) }))
        .route("/trade-stats", get(list_trade_stats))
        .with_state(Arc::new(db))
}

async fn list_trade_stats(State(db): State<Arc<PgPool>>) -> Json<serde_json::Value> {
    match fetch_trade_stats(&db).await {
        Ok(stats) => Json(serde_json::json!({ "data": stats })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}
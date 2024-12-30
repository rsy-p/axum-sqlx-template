mod config;
mod database;
mod env;
mod redis;
mod tracing;

use server_core::AppState;

/// 初始化资源
pub async fn init() -> anyhow::Result<AppState> {
    env::init();
    tracing::init();
    let config = config::init();
    let db = database::init(&config).await?;
    let redis = redis::init(&config).await?;
    let state = AppState::new(config, db, redis);
    Ok(state)
}

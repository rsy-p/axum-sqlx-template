use bb8_redis::RedisConnectionManager;
use server_core::{Config, RedisPool};

/// 加载redis连接池
pub async fn init(config: &Config) -> anyhow::Result<RedisPool> {
    let redis_manager = RedisConnectionManager::new(&*config.redis_url)?;
    let pool = bb8::Pool::builder()
        .max_size(config.max_redis_pool)
        .build(redis_manager)
        .await?;
    Ok(pool)
}

use crate::{Config, RedisConn, RedisPool};
use sqlx::{PgPool, Postgres, Transaction};

pub struct AppState {
    pub config: Config,
    pub db: PgPool,
    redis: RedisPool,
}

impl AppState {
    pub fn new(config: Config, db: PgPool, redis: RedisPool) -> Self {
        AppState { config, db, redis }
    }

    /// 获取redis连接
    pub async fn redis_conn(&self) -> anyhow::Result<RedisConn> {
        Ok(self.redis.get().await?)
    }

    /// 获取pg连接
    pub fn pg_conn(&self) -> &PgPool {
        &self.db
    }

    /// 获取pg事务
    pub async fn pg_tx(&self) -> anyhow::Result<Transaction<Postgres>> {
        Ok(self.db.begin().await?)
    }
}

use crate::AppError;
use bb8::{Pool, PooledConnection};
use bb8_redis::RedisConnectionManager;

/// Redis类型
pub type RedisPool = Pool<RedisConnectionManager>;
pub type RedisConn<'a> = PooledConnection<'a, RedisConnectionManager>;

/// 结果类型
pub type Result<T> = std::result::Result<T, AppError>;

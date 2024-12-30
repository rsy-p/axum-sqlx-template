use redis::AsyncCommands;
use server_core::{RedisConn, Result};

pub async fn set(mut redis_conn: RedisConn<'_>) -> Result<()> {
    let _: () = redis_conn.set("foo", "dss").await?;
    Ok(())
}

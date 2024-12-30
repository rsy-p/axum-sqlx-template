use server_core::Config;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

/// 加载数据库连接池
pub async fn init(config: &Config) -> anyhow::Result<PgPool> {
    let db = PgPoolOptions::new()
        .max_connections(config.max_database_pool)
        .connect(&config.database_url)
        .await?;
    // 运行数据库迁移脚本
    sqlx::migrate!("../migrations").run(&db).await?;
    Ok(db)
}

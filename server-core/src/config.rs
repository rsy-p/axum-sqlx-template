/// 配置信息
#[derive(clap::Parser)]
pub struct Config {
    /// 端口
    #[clap(long, env, default_value = "8080")]
    pub server_port: u16,

    /// 上下文路径
    #[clap(long, env, default_value = "/")]
    pub context_path: String,

    /// 公开路径
    #[clap(long, env, default_value = "/")]
    pub public_path: String,

    /// JWT 密钥
    #[clap(long, env)]
    pub jwt_secret: String,

    /// JWT 过期时间
    #[clap(long, env)]
    pub jwt_exp: usize,

    /// 数据库连接地址
    #[clap(long, env)]
    pub database_url: String,

    /// 数据库最大连接数
    #[clap(long, env, default_value = "10")]
    pub max_database_pool: u32,

    /// Redis连接地址
    #[clap(long, env)]
    pub redis_url: String,

    /// Redis 最大连接数
    #[clap(long, env, default_value = "10")]
    pub max_redis_pool: u32,
}

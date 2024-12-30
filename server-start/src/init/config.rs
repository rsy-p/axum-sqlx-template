use clap::Parser;
use server_core::Config;

/// 加载配置
pub fn init() -> Config {
    Config::parse()
}

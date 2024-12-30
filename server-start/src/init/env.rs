/// 加载.env文件
pub fn init() {
    dotenvy::dotenv().ok();
}

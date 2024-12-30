mod init;

use init::init;
use std::net::SocketAddr;
use std::sync::Arc;

/// 启动服务
pub async fn start() -> anyhow::Result<()> {
    let state = init().await?;
    let addr = SocketAddr::from(([0, 0, 0, 0], state.config.server_port));
    tracing::info!("Server listening on {}", addr);
    let state = Arc::new(state);
    let app = server_api::router(state.clone());
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

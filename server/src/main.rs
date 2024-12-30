#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server_start::start().await?;
    Ok(())
}

use anyhow;
mod core;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    core::setup().await?;

    Ok(())
}

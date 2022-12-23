use anyhow;
use std::path::Path;
mod core;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = Path::new("./algovault.sqlite");
    core::setup(path).await?;
    Ok(())
}

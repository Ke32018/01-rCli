use anyhow::Result;
use clap::Parser;
use rcli::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    args.cmd.execute().await?;
    Ok(())
}

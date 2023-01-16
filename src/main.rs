use anyhow::Result;
use kube::Client;
use log::info;
use simple_logger::SimpleLogger;
use time::UtcOffset;

use rbacr::{Access, Check};

#[tokio::main]
async fn main() -> Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .with_utc_offset(UtcOffset::from_hms(8, 0, 0).unwrap())
        .init()
        .unwrap();

    info!("Start rbac-rs");
    let client = Client::try_default().await?;
    let access = Access::run(client).await?;
    let content = include_str!("../fixtures/check.yaml");
    let check = Check::from_yaml(content)?;
    check.run(access).await?;

    Ok(())
}

use anyhow::Result;
use rbacr::Check;

#[tokio::main]
async fn main() -> Result<()> {
    let content = include_str!("../fixtures/check.yaml");
    let check = Check::from_yaml(content)?;

    println!("{:#?}", check);
    println!("{:?}", check.get_check()?);
    Ok(())
}

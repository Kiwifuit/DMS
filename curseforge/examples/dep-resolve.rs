use anyhow::Context;
use curseforge::types::CurseResponse;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv().context("while loading dotenv")?;

  let resp = CurseResponse::new("test");

  Ok(())
}

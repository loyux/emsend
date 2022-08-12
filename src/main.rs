use anyhow::Error;
use posmtp::cli_run;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    cli_run().await?;
    Ok(())
}

use std::error::Error;
use clap::Parser;
pub mod cli;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    dotenv::dotenv().ok();
    env_logger::init();
    let cli = cli::Cli::parse();
    cli.execute().await
}

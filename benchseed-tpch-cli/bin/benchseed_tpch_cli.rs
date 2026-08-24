use benchseed_cli::tpch_cli::Cli;
use clap::Parser;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    Cli::parse().run().await
}

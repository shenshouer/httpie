use anyhow::Result;
use clap::Parser;
use httpie::Opts;

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    opts.exec().await
}

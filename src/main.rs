use clap::Parser;
use rcli::{CmdExecutor, Opts};

// rcli csv -i input.csv -o output.json --header --pretty -d ','
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts: Opts = Opts::parse();

    opts.cmd.execute().await?;

    Ok(())
}

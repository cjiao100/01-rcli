use crate::CmdExecutor;

use super::verify_path;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub enum HTTPSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HTTPServeOpts),
}

#[derive(Debug, Parser)]
pub struct HTTPServeOpts {
    #[arg(short, long, value_parser=verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExecutor for HTTPServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        crate::process_http_serve(self.dir, self.port).await
    }
}

impl CmdExecutor for HTTPSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HTTPSubCommand::Serve(opts) => opts.execute().await,
        }
    }
}

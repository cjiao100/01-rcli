mod base64_opts;
mod csv_opts;
mod gen_pass_opts;
mod http_opts;
mod jwt_opts;
mod text_opts;

use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::{Path, PathBuf};

pub use self::{
    base64_opts::*, csv_opts::*, gen_pass_opts::*, http_opts::*, jwt_opts::*, text_opts::*,
};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum SubCommand {
    #[command(name = "csv", about = "show CSV , or Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Text sign/verify")]
    Text(TextSubCommand),
    #[command(subcommand, about = "HTTP serve")]
    Http(HTTPSubCommand),
    #[command(subcommand, about = "JWT sign/verify")]
    JWT(JWTSubCommand),
}

// impl CmdExecutor for SubCommand {
//     async fn execute(self) -> anyhow::Result<()> {
//         match self {
//             SubCommand::Csv(opts) => opts.execute().await,
//             SubCommand::GenPass(opts) => opts.execute().await,
//             SubCommand::Base64(cmd) => cmd.execute().await,
//             SubCommand::Text(cmd) => cmd.execute().await,
//             SubCommand::Http(cmd) => cmd.execute().await,
//         }
//     }
// }

fn verify_file(filename: &str) -> Result<String, &'static str> {
    // 判断 filename 是否为 "-" 或者文件是否存在
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File not found")
    }
}

fn verify_path(filename: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(filename);
    if p.exists() && p.is_dir() {
        Ok(filename.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

// cargo test --test cli
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File not found"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not_found.csv"), Err("File not found"));
    }
}

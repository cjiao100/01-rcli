mod base64_opts;
mod csv_opts;
mod gen_pass_opts;

use clap::Parser;
use std::path::Path;

pub use base64_opts::Base64SubCommand;
use csv_opts::CsvOpts;
use gen_pass_opts::GenPassOpts;

// pub use self::csv_opts::OutputFormat; 等价写法
pub use base64_opts::Base64Format;
pub use csv_opts::OutputFormat;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "show CSV , or Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    // 判断 filename 是否为 "-" 或者文件是否存在
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File not found")
    }
}

// cargo test --test cli
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File not found"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("not_found.csv"), Err("File not found"));
    }
}

use super::verify_file;
use clap::Parser;
use std::{fmt, str::FromStr};

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode base64")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    // "-" 表示从标准输入读取数据，stdin
    #[arg(short, long, value_parser=verify_file, help = "Input string", default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard", help = "Base64 format")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    // "-" 表示从标准输入读取数据，stdin
    #[arg(short, long, value_parser=verify_file, help = "Input base64 string", default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard", help = "Base64 format")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid base64 format")),
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Base64Format::Standard => write!(f, "standard"),
            Base64Format::UrlSafe => write!(f, "urlsafe"),
        }
    }
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

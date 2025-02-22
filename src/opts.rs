use clap::Parser;
use std::{
    fmt::{self},
    path::Path,
    str::FromStr,
};

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
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input CSV file path", value_parser = verify_input_file)]
    pub input: String,

    // "output.json".into() 会将字符串转换为String类型
    // Option<String> 表示这个字段是可选的
    #[arg(short, long, help = "Output file path")]
    pub output: Option<String>,

    #[arg(short, long, help = "CSV delimiter", default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, help = "Output file format", value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, help = "Pretty print JSON output")]
    pub pretty: bool,

    #[arg(long, help = "CSV file has header", default_value_t = true)]
    pub header: bool,
}

pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File not found")
    }
}

pub fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    // parse 可以将字符串转换为指定类型，前提是需要实现FromStr
    format.parse()
}

// From 是一个trait，用于将指定类型转换为其他类型，会自动生成Into 的实现
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

// FromStr 是一个trait，用于将字符串转换为指定类型
impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            v => anyhow::bail!("Unsupported format: {}", v),
        }
    }
}

// fmt::Display 是一个trait，用于将指定类型转换为字符串
// fmt::Result 是一个类型，用于处理格式化字符串的结果
// fmt::Formatter 是一个类型，用于格式化字符串
// write! 是一个宏，用于将指定类型转换为字符串
// write!(f, "{}", Into::<&str>::into(*self)) 将OutputFormat转换为字符串
// *self 表示将self解引用
impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

use clap::Parser;
use std::path::Path;

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

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input CSV file path", value_parser = verify_input_file)]
    pub input: String,

    // "output.json".into()
    #[arg(short, long, help = "Output file path", default_value = "output.json")]
    pub output: String,

    #[arg(short, long, help = "CSV delimiter", default_value_t = ',')]
    pub delimiter: char,

    #[arg(short, long, help = "Output file format")]
    pub format: Option<String>,

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

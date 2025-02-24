use super::{verify_file, verify_path};
use clap::Parser;
use std::{fmt, path::PathBuf, str::FromStr};

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signed message")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a new key")]
    Generate(TextKeyGenerateOpts),
    #[command(about = "Encrypt a message")]
    Encrypt(TextEncryptOpts),
    #[command(about = "Decrypt a message")]
    Decrypt(TextDecryptOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser=verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser=verify_file)]
    pub key: String,
    #[arg(long, default_value = "black3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser=verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser=verify_file)]
    pub key: String,
    #[arg(long, default_value = "black3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long)]
    pub sig: String,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, default_value = "black3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    // #[arg(short, long, value_parser=verify_file, default_value = "-")]
    // pub input: String,
    #[arg(short, long, value_parser=verify_file)]
    pub key: String,
    #[arg(short, long, value_parser=verify_file)]
    pub nonce: String,
}

#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    // #[arg(short, long, value_parser=verify_file, default_value = "-")]
    // pub input: String,
    #[arg(short, long, value_parser=verify_file)]
    pub key: String,
    #[arg(short, long, value_parser=verify_file)]
    pub nonce: String,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
    ChaChaPoly,
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "black3",
            TextSignFormat::Ed25519 => "ed25519",
            TextSignFormat::ChaChaPoly => "chacha_poly",
        }
    }
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "black3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            "chacha_poly" => Ok(TextSignFormat::ChaChaPoly),
            _ => Err(anyhow::anyhow!("Invalid Text Sign format")),
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextSignFormat::Blake3 => write!(f, "black3"),
            TextSignFormat::Ed25519 => write!(f, "ed25519"),
            TextSignFormat::ChaChaPoly => write!(f, "chacha_poly"),
        }
    }
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

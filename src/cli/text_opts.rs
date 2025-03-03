use crate::CmdExecutor;

use super::{verify_file, verify_path};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::{fmt, path::PathBuf, str::FromStr};
use tokio::fs;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
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

impl CmdExecutor for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        crate::process_text_sign(&self.input, &self.key, self.format)?;

        Ok(())
    }
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        crate::process_text_verify(&self.input, &self.key, self.format, &self.sig)?;
        Ok(())
    }
}

impl CmdExecutor for TextKeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = crate::process_text_generate(self.format)?;

        match self.format {
            TextSignFormat::Blake3 => {
                // 保存到文件
                let name = self.output.join("blake3.txt");
                fs::write(name, &key[0]).await?;
            }
            TextSignFormat::Ed25519 => {
                // 保存到文件
                let name = self.output.join("ed25519.txt");
                fs::write(name, &key[0]).await?;

                let name = self.output.join("ed25519_verifier.txt");
                fs::write(name.join("ed25519_verifier.txt"), &key[1]).await?;
            }
            TextSignFormat::ChaChaPoly => {
                // 保存到文件
                let name = self.output.join("chacha_poly.txt");
                fs::write(name, &key[0]).await?;

                let name = self.output.join("chacha_poly.nonce");
                fs::write(name.join("name"), &key[1]).await?;
            }
        }

        Ok(())
    }
}

impl CmdExecutor for TextEncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encrypted = crate::process_text_encrypt(&self.key, &self.nonce)?;
        println!("{}", encrypted);
        Ok(())
    }
}

impl CmdExecutor for TextDecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decrypted = crate::process_text_decrypt(&self.key, &self.nonce)?;
        println!("{}", decrypted);
        Ok(())
    }
}

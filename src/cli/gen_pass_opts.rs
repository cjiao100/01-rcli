use anyhow::Ok;
use clap::Parser;
use zxcvbn::zxcvbn;

use crate::CmdExecutor;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, help = "Password length", default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = false)]
    pub no_uppercase: bool,

    #[arg(long, default_value_t = false)]
    pub no_lowercase: bool,

    #[arg(long, default_value_t = false)]
    pub no_number: bool,

    #[arg(long, default_value_t = false)]
    pub no_symbol: bool,
}

impl CmdExecutor for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let password = crate::process_gen_pass(
            self.length,
            !self.no_uppercase,
            !self.no_lowercase,
            !self.no_number,
            !self.no_symbol,
        )?;

        println!("password: {}", password);

        let estimate = zxcvbn(&password, &[]);
        eprintln!("score: {}", estimate.score());
        Ok(())
    }
}

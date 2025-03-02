mod cli;
mod process;
mod utils;

pub use cli::{
    Base64Format, Base64SubCommand, HTTPSubCommand, Opts, OutputFormat, SubCommand, TextSignFormat,
    TextSubCommand,
};

pub use process::{
    process_csv, process_decode, process_encode, process_gen_pass, process_http_serve,
    process_text_decrypt, process_text_encrypt, process_text_generate, process_text_sign,
    process_text_verify,
};

pub use utils::*;

#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}

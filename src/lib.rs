mod cli;
mod process;

pub use cli::{Base64Format, Base64SubCommand, Opts, OutputFormat, SubCommand};

pub use process::{process_csv, process_decode, process_encode, process_gen_pass};

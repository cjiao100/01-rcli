use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_gen_pass, Base64SubCommand, Opts,
    SubCommand,
};

// rcli csv -i input.csv -o output.json --header --pretty -d ','

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                // format! 宏会调用Display trait, 将OutputFormat转换为字符串
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            let password = process_gen_pass(
                opts.length,
                !opts.no_uppercase,
                !opts.no_lowercase,
                !opts.no_number,
                !opts.no_symbol,
            );

            println!("password: {}", password?);
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
                // println!("{:?}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
                // println!("{:?}", decoded);
            }
        },
    }

    Ok(())
}

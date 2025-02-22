use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

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
    }

    Ok(())
}

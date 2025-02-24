use std::fs;

use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_gen_pass, process_text_decrypt,
    process_text_encrypt, process_text_generate, process_text_sign, process_text_verify,
    Base64SubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand,
};
use zxcvbn::zxcvbn;

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
            )?;

            println!("password: {}", password);

            // 打印密码强度
            let estimate = zxcvbn(&password, &[]);
            eprintln!("score: {}", estimate.score());
        }
        SubCommand::Base64(sub_cmd) => match sub_cmd {
            Base64SubCommand::Encode(opts) => {
                let encoded = process_encode(&opts.input, opts.format)?;
                print!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;

                print!("{}", String::from_utf8(decoded)?);
            }
        },
        SubCommand::Text(sub_cmd) => match sub_cmd {
            TextSubCommand::Sign(opts) => {
                let signed = process_text_sign(&opts.input, &opts.key, opts.format)?;

                println!("{}", signed);
            }
            TextSubCommand::Verify(opts) => {
                let verified = process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;

                println!("{}", verified);
            }
            TextSubCommand::Generate(opts) => {
                let key = process_text_generate(opts.format)?;
                // println!("{}", String::from_utf8(key.concat())?);

                match opts.format {
                    TextSignFormat::Blake3 => {
                        // 保存到文件
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        // 保存到文件
                        let name = &opts.output;

                        fs::write(name.join("ed25519_signer.txt"), &key[0])?;
                        fs::write(name.join("ed25519_verifier.txt"), &key[1])?;
                    }
                    TextSignFormat::ChaChaPoly => {
                        // 保存到文件
                        let name = &opts.output;
                        fs::write(name.join("chachaPoly.key"), &key[0])?;
                        fs::write(name.join("chachaPoly.nonce"), &key[1])?;
                    }
                }
            }
            TextSubCommand::Encrypt(opts) => {
                let encrypted = process_text_encrypt(&opts.key, &opts.nonce)?;

                println!("{}", encrypted);
            }
            TextSubCommand::Decrypt(opts) => {
                let decrypted = process_text_decrypt(&opts.key, &opts.nonce)?;

                println!("{}", decrypted);
            }
        },
    }

    Ok(())
}

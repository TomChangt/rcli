use std::fs;

use clap::Parser;
use rcli::{
    get_reader, process_csv, process_decode, process_encode, process_genpass, process_http_serve,
    process_text_generate, process_text_sign, process_text_verify, Base64SubCommand,
    HttpSubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand,
};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::Genpass(opts) => {
            let pwd = process_genpass(
                opts.length,
                opts.no_uppercase,
                opts.no_lowercase,
                opts.no_numbers,
                opts.no_symbols,
            )?;
            println!("Generated password: {}", pwd);
        }
        SubCommand::Base64(subcommand) => match subcommand {
            Base64SubCommand::Encode(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let encoded = process_encode(&mut reader, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let decoded = process_decode(&mut reader, opts.format)?;
                info!("{}", String::from_utf8(decoded)?);
            }
        },
        SubCommand::Text(subcommand) => match subcommand {
            TextSubCommand::Sign(opts) => {
                let signed = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", signed);
            }
            TextSubCommand::Verify(opts) => {
                let verified =
                    process_text_verify(&opts.input, &opts.key, opts.format, &opts.sign)?;
                println!("{}", verified);
            }
            TextSubCommand::Generate(opts) => {
                let key = process_text_generate(opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let path = opts.output.join("blake3.key");
                        fs::write(path, &key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = opts.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
        },
        SubCommand::Http(subcommand) => match subcommand {
            HttpSubCommand::Serve(opts) => {
                process_http_serve(opts.dir, opts.port).await?;
            }
        },
    }
    Ok(())
}

use clap::Parser;
use rcli::{
    get_reader, process_csv, process_decode, process_encode, process_genpass, Base64SubCommand,
    Opts, SubCommand,
};

fn main() -> anyhow::Result<()> {
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
                println!("Encoded: {}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let decoded = process_decode(&mut reader, opts.format)?;
                println!("Decoded: {:?}", decoded);
            }
        },
    }
    Ok(())
}

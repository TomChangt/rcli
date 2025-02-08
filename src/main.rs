use clap::Parser;
use rcli::{process_csv, process_genpass, Opts, SubCommand};

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
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_example() {
        // 测试代码
        assert_eq!(2 + 2, 4);
    }
}

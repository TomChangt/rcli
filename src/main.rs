use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
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

use crate::{process_genpass, CmdExecutor};
use clap::Parser;
use zxcvbn::zxcvbn;
#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = false)]
    pub no_symbols: bool,

    #[arg(long, default_value_t = false)]
    pub no_lowercase: bool,

    #[arg(long, default_value_t = false)]
    pub no_uppercase: bool,

    #[arg(long, default_value_t = false)]
    pub no_numbers: bool,
}

impl CmdExecutor for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let pwd = process_genpass(
            self.length,
            self.no_uppercase,
            self.no_lowercase,
            self.no_numbers,
            self.no_symbols,
        )?;
        println!("{}", pwd);

        // output password strength in stderr
        let estimate = zxcvbn(&pwd, &[])?;
        eprintln!("Password strength: {}", estimate.score());
        Ok(())
    }
}

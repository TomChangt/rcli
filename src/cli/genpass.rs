use clap::Parser;
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

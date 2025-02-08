mod base64;
mod csv;
mod genpass;

use std::path::Path;

pub use self::{base64::*, csv::*, genpass::*};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show csv , or convert csv to other format")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate random password")]
    Genpass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode or decode")]
    Base64(Base64SubCommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

// fn verify_path(filename: &str) -> Result<PathBuf, &'static str> {
//     let p = Path::new(filename);
//     if p.exists() && p.is_dir() {
//         Ok(p.into())
//     } else {
//         Err("Path does not exist or is not a directory".into())
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File does not exist"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File does not exist"));
    }
}

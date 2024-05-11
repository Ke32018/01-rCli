mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use self::{csv::CsvOpts, genpass::GenPassOpts};
use clap::Parser;
use std::path::{Path, PathBuf};

pub use self::{
    base64::{Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64SubCommand},
    csv::OutputFormat,
    http::{HttpServeOpts, HttpSubCommand},
    text::{TextSignFormat, TextSignOpts, TextSubCommand, TextVerifyOpts},
};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
    #[command(subcommand, about = "Serve a dir over HTTP")]
    Http(HttpSubCommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    // if input is "-" or file exists
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

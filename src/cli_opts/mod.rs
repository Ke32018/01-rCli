mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::{Path, PathBuf};

pub use self::{base64::*, csv::*, genpass::*, http::*, text::*};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Text sign/verify")]
    Text(TextSubCommand),
    #[command(subcommand, about = "Serve a dir over HTTP")]
    Http(HttpSubCommand),
}

// impl CmdExector for SubCommand {
//     async fn execute(self) -> anyhow::Result<()> {
//         match self {
//             SubCommand::Csv(opts) => opts.execute().await,
//             SubCommand::GenPass(opts) => opts.execute().await,
//             SubCommand::Base64(cmd) => cmd.execute().await,
//             SubCommand::Text(cmd) => cmd.execute().await,
//             SubCommand::Http(cmd) => cmd.execute().await,
//         }
//     }
// }

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

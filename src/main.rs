use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, Args, SubCommand};

fn main() -> Result<()> {
    let args = Args::parse();
    match args.cmd {
        SubCommand::Csv(opts) => {
            process_csv(&opts.input, &opts.output)?;
        }
    }
    Ok(())
}

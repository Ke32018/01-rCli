use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, Args, SubCommand};

fn main() -> Result<()> {
    let args = Args::parse();
    match args.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
    }
    Ok(())
}

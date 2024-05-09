use anyhow::Result;
use clap::Parser;
use rcli::*;

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
        SubCommand::GenPass(opts) => {
            let pass = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("{}", pass);
        }
        SubCommand::Base64(subcmd) => {
            // println!("{:?}", cmd);
            match subcmd {
                Base64SubCommand::Encode(opts) => {
                    let mut reader = get_reader(&opts.input)?;
                    let res = process_encode(&mut reader, opts.format)?;
                    println!("{}", res);
                }
                Base64SubCommand::Decode(opts) => {
                    let mut reader = get_reader(&opts.input)?;
                    let res = process_decode(&mut reader, opts.format)?;
                    println!("{}", res);
                }
            }
        }
    }
    Ok(())
}

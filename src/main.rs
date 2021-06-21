mod args;
use args::Args;
use structopt::StructOpt;
use anyhow::{Context, Result, Error};

fn cause_error(reason: &'static str) -> Result<()> {
    Err(Error::msg(reason))
}

fn main() -> Result<()> {
    let args = Args::from_args(); 
    println!("{}", args);
    cause_error("Error caused to check out error handling").context("`cause_error` called in main function")?;
    Ok(())
}

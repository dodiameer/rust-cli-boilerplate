use std::fmt;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Verbose: {}", self.verbose)
    }
}

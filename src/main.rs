use clap::Parser;
use stter::Cli;

fn main() {
    let args = Cli::parse();

    stter::read(&args);
}

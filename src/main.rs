use stter::Cli;
use clap::Parser;

fn main() {
    let args = Cli::parse();

    stter::read(&args);
}

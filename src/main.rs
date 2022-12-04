mod common;
mod days;
use clap::Parser;
use days::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    day_command: DayCommand,
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = cli.day_command.run() {
        eprintln!("command failed with error {:?}", e);
    }
}

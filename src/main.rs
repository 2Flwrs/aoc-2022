mod common;
mod days;
use clap::Parser;
use days::PuzzleArgs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    puzzle_args: PuzzleArgs,
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = cli.puzzle_args.run() {
        eprintln!("command failed with error {:?}", e);
    }
}

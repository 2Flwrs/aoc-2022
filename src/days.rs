pub mod day1;
pub use day1::Day1Arg;

pub mod day2;
pub use day2::Day2Arg;

pub mod day3;
pub use day3::Day3Arg;

pub mod day4;
pub use day4::Day4Arg;

use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum DayCommand {
    Day1(Day1Arg),
    Day2(Day2Arg),
    Day3(Day3Arg),
    Day4(Day4Arg),
}

impl DayCommand {
    pub(crate) fn run(self) -> Result<()> {
        match self {
            DayCommand::Day1(arg) => arg.run(),
            DayCommand::Day2(arg) => arg.run(),
            DayCommand::Day3(arg) => arg.run(),
            DayCommand::Day4(arg) => arg.run(),
        }
    }
}
